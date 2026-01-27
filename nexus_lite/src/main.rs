use axum::{
    extract::State,
    response::{sse::{Event, Sse}, Html},
    routing::{get, post},
    Json, Router,
};
use tower_http::services::ServeDir;
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use std::{fs, sync::{Arc, Mutex}, path::Path};
use sysinfo::System;
use std::time::Duration;
use chrono::Local;

// --- CONFIGURATION ---
const MODEL_NAME: &str = "llama3.2:1b"; 
const OLLAMA_URL: &str = "http://127.0.0.1:11434/api/chat";
const SYSTEM_PROMPT: &str = "You are NEXUS. Be concise. Output JSON for actions: ```json\n[{\"action\": \"SAVE\", \"path\": \"file.txt\", \"content\": \"...\"}]\n```";

#[derive(Clone, Serialize, Deserialize, Debug)]
struct Message { role: String, content: String, timestamp: String }

#[derive(Deserialize)]
struct AskRequest { prompt: String }
#[derive(Deserialize)]
struct FileRequest { path: String, content: String, #[serde(default)] force: bool }
#[derive(Deserialize)]
struct CreateRequest { filename: String }
#[derive(Deserialize)]
struct DeleteRequest { filename: String }
#[derive(Deserialize)]
struct RenameRequest { old_name: String, new_name: String }

struct AppState {
    client: reqwest::Client,
    history: Mutex<Vec<Message>>,
    tx_monitor: tokio::sync::broadcast::Sender<String>
}

#[tokio::main]
async fn main() {
    let _ = fs::create_dir_all("workspace");
    let initial_history = load_memory();
    let (tx, _) = tokio::sync::broadcast::channel(100);
    let state = Arc::new(AppState {
        client: reqwest::Client::new(),
        history: Mutex::new(initial_history),
        tx_monitor: tx.clone(),
    });

    let tx_clone = tx.clone();
    tokio::spawn(async move {
        let mut sys = System::new_all();
        loop {
            sys.refresh_all();
            let json = serde_json::json!({
                "cpu": format!("{:.0}%", sys.global_cpu_info().cpu_usage()),
                "ram": format!("{:.1}GB", sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0),
                "uptime": format!("{}s", System::uptime())
            });
            let _ = tx_clone.send(json.to_string());
            tokio::time::sleep(Duration::from_secs(2)).await;
        }
    });

    let app = Router::new()
        .route("/", get(|| async { Html(fs::read_to_string("index.html").unwrap_or_default()) }))
        .route("/ask", post(ask_stream))
        .route("/monitor", get(monitor_stream))
        .route("/files", get(list_files))
        .route("/save", post(save_file))
        .route("/create", post(create_file))
        .route("/delete", post(delete_file))
        .route("/rename", post(rename_file))
        .nest_service("/workspace", ServeDir::new("workspace"))
        .with_state(state);

    println!("🚀 NEXUS LITE // MODEL: {}", MODEL_NAME);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8090").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn load_memory() -> Vec<Message> {
    if let Ok(data) = fs::read_to_string("workspace/memory.json") { serde_json::from_str(&data).unwrap_or_else(|_| Vec::new()) } else { Vec::new() }
}
fn save_memory(history: &Vec<Message>) {
    let _ = fs::write("workspace/memory.json", serde_json::to_string_pretty(history).unwrap_or_default());
}

async fn monitor_stream(State(state): State<Arc<AppState>>) -> Sse<impl futures_util::Stream<Item = Result<Event, std::convert::Infallible>>> {
    let mut rx = state.tx_monitor.subscribe();
    Sse::new(async_stream::stream! { while let Ok(msg) = rx.recv().await { yield Ok(Event::default().data(msg)); } })
}

async fn list_files() -> Json<Vec<String>> {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir("workspace") {
        for entry in entries.flatten() {
            if let Ok(name) = entry.file_name().into_string() { if name != "memory.json" { files.push(name); } }
        }
    }
    Json(files)
}
async fn save_file(Json(req): Json<FileRequest>) -> Json<serde_json::Value> {
    let path = Path::new("workspace").join(Path::new(&req.path).file_name().unwrap_or_default());
    if path.exists() && !req.force { return Json(serde_json::json!({ "status": "exists" })); }
    match fs::write(path, req.content) { Ok(_) => Json(serde_json::json!({ "status": "success" })), Err(e) => Json(serde_json::json!({ "status": "error", "details": e.to_string() })) }
}
async fn create_file(Json(req): Json<CreateRequest>) -> Json<serde_json::Value> {
    let path = Path::new("workspace").join(Path::new(&req.filename).file_name().unwrap_or_default());
    if path.exists() { return Json(serde_json::json!({ "status": "error", "details": "Exists" })); }
    match fs::write(path, "") { Ok(_) => Json(serde_json::json!({ "status": "success" })), Err(e) => Json(serde_json::json!({ "status": "error", "details": e.to_string() })) }
}
async fn delete_file(Json(req): Json<DeleteRequest>) -> Json<serde_json::Value> {
    let path = Path::new("workspace").join(Path::new(&req.filename).file_name().unwrap_or_default());
    match fs::remove_file(path) { Ok(_) => Json(serde_json::json!({ "status": "success" })), Err(e) => Json(serde_json::json!({ "status": "error", "details": e.to_string() })) }
}
async fn rename_file(Json(req): Json<RenameRequest>) -> Json<serde_json::Value> {
    let old = Path::new("workspace").join(Path::new(&req.old_name).file_name().unwrap_or_default());
    let new = Path::new("workspace").join(Path::new(&req.new_name).file_name().unwrap_or_default());
    match fs::rename(old, new) { Ok(_) => Json(serde_json::json!({ "status": "success" })), Err(e) => Json(serde_json::json!({ "status": "error", "details": e.to_string() })) }
}

async fn ask_stream(State(state): State<Arc<AppState>>, Json(req): Json<AskRequest>) -> Sse<impl futures_util::Stream<Item = Result<Event, std::convert::Infallible>>> {
    // PREPARE MESSAGES (LOCK TEMPORARILY)
    let messages = {
        let mut hist = state.history.lock().unwrap();
        hist.push(Message { role: "user".into(), content: req.prompt.clone(), timestamp: Local::now().to_rfc3339() });
        save_memory(&hist);
        let mut msgs = vec![serde_json::json!({ "role": "system", "content": SYSTEM_PROMPT })];
        for msg in hist.iter().rev().take(6).rev() { msgs.push(serde_json::json!({ "role": &msg.role, "content": &msg.content })); }
        msgs
    };

    let client = state.client.clone();
    let state_clone = state.clone();

    let stream = async_stream::stream! {
        let payload = serde_json::json!({ "model": MODEL_NAME, "messages": messages, "stream": true });
        let mut full_resp = String::new();

        if let Ok(res) = client.post(OLLAMA_URL).json(&payload).send().await {
            let mut byte_stream = res.bytes_stream();
            while let Some(Ok(bytes)) = byte_stream.next().await {
                for line in String::from_utf8_lossy(&bytes).lines() {
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(line) {
                        if let Some(c) = json["message"]["content"].as_str() {
                            full_resp.push_str(c);
                            yield Ok(Event::default().data(c));
                        }
                    }
                }
            }
        }
        let mut hist = state_clone.history.lock().unwrap();
        hist.push(Message { role: "assistant".into(), content: full_resp, timestamp: Local::now().to_rfc3339() });
        save_memory(&hist);
    };
    Sse::new(stream)
}
