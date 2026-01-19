use axum::{
    extract::State,
    response::{sse::{Event, Sse}, Html},
    routing::{get, post},
    Json, Router,
};
use tower_http::services::ServeDir;
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use std::{fs, sync::{Arc, Mutex}};
use tokio::process::Command;
use std::process::Stdio;

// --- CONFIG ---
// We instruct the AI to output JSON for actions, which the Frontend will catch
const SYSTEM_PROMPT: &str = "You are NEXUS. \
If the user asks to run code or save files, output strictly valid JSON at the end of your response. \
Format: [{\"action\": \"RUN\", \"cmd\": \"...\"}] or [{\"action\": \"SAVE\", \"path\": \"...\", \"content\": \"...\"}] \
Do not markdown format the JSON. Do not explain the JSON.";

// --- STATE ---
#[derive(Clone, Serialize, Deserialize, Debug)]
struct Message { role: String, content: String }

struct AppState {
    client: reqwest::Client,
    history: Mutex<Vec<Message>>,
}

#[derive(Deserialize)]
struct AskRequest { prompt: String }
#[derive(Deserialize)]
struct ExecRequest { cmd: String }

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    // Ensure workspace exists
    let _ = fs::create_dir("workspace");

    let state = Arc::new(AppState {
        client: reqwest::Client::new(),
        history: Mutex::new(Vec::new()),
    });

    let app = Router::new()
        .route("/", get(index))
        .route("/ask", post(ask_stream))
        .route("/execute", post(execute_cmd)) // The "Green Button" hits this
        .nest_service("/workspace", ServeDir::new("workspace"))
        .with_state(state);

    println!("🚀 NEXUS MOBILE NODE // PORT 8090");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8090").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> Html<String> {
    Html(fs::read_to_string("index.html").unwrap_or_else(|_| "<h1>Error: Missing index.html</h1>".to_string()))
}

// --- COMMAND EXECUTION (The Hand) ---
// This runs only when the user clicks AUTHORIZE on the frontend
async fn execute_cmd(Json(req): Json<ExecRequest>) -> Json<serde_json::Value> {
    println!("> EXECUTING: {}", req.cmd);
    // Use sh -c to allow piping and arguments
    let output = Command::new("sh").arg("-c").arg(&req.cmd).output().await;
    match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout).to_string();
            let stderr = String::from_utf8_lossy(&out.stderr).to_string();
            Json(serde_json::json!({ "success": true, "stdout": stdout, "stderr": stderr }))
        },
        Err(e) => Json(serde_json::json!({ "success": false, "error": e.to_string() })),
    }
}

// --- AI STREAM (The Brain) ---
async fn ask_stream(State(state): State<Arc<AppState>>, Json(req): Json<AskRequest>) -> Sse<impl futures_util::Stream<Item = Result<Event, std::convert::Infallible>>> {
    // 1. Update Memory
    {
        let mut hist = state.history.lock().unwrap();
        hist.push(Message { role: "user".into(), content: req.prompt.clone() });
    }

    let client = state.client.clone();
    let state_clone = state.clone();

    let stream = async_stream::stream! {
        let api_key = std::env::var("GEMINI_API_KEY").unwrap_or_default();
        let history_guard = state_clone.history.lock().unwrap();
        
        // Build Context
        let mut contents = vec![
            serde_json::json!({ "role": "user", "parts": [{ "text": SYSTEM_PROMPT }] })
        ];
        
        // Add recent history (Limit 8 turns)
        for msg in history_guard.iter().rev().take(8).rev() {
             let role = if msg.role == "user" { "user" } else { "model" };
             contents.push(serde_json::json!({ "role": role, "parts": [{ "text": msg.content }] }));
        }
        drop(history_guard);

        let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:streamGenerateContent?alt=sse&key={}", api_key);
        let payload = serde_json::json!({ "contents": contents });

        let mut full_text = String::new();

        if let Ok(res) = client.post(&url).json(&payload).send().await {
            let mut byte_stream = res.bytes_stream();
            while let Some(Ok(bytes)) = byte_stream.next().await {
                let chunk = String::from_utf8_lossy(&bytes);
                for line in chunk.lines() {
                    if line.starts_with("data: ") {
                        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&line[6..]) {
                            if let Some(text) = json["candidates"][0]["content"]["parts"][0]["text"].as_str() {
                                full_text.push_str(text);
                                yield Ok(Event::default().data(text));
                            }
                        }
                    }
                }
            }
        }
        
        // Save AI response to memory
        state_clone.history.lock().unwrap().push(Message { role: "model".into(), content: full_text });
    };
    Sse::new(stream)
}
