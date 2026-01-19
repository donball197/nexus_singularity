use axum::{
    extract::State,
    response::{sse::{Event, Sse}, Html},
    routing::{get, post},
    Json, Router,
};
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use std::{convert::Infallible, fs, process::Command, sync::{Arc, Mutex}};
use sysinfo::{System, SystemExt, CpuExt};

// --- CONFIGURATION ---
// Updated to User Specification
const GEMINI_MODEL: &str = "models/gemma-3-27b-it";
const API_VERSION: &str = "v1beta";
const SYSTEM_INSTRUCTION: &str = "You are NEXUS. Mobile-First. You have access to the 'workspace/' directory. \
1. To read a file: [READ: filename] \
2. To save code: [SAVE: filename] ...code... [/SAVE] \
3. To run terminal commands: [EXEC: command] \
(Only run commands if necessary. If you read a file, I will show you the content in the next turn).";

#[derive(Deserialize)]
struct AskRequest { prompt: String, editor_context: Option<String> }
#[derive(Deserialize)]
struct CmdRequest { cmd: String }
#[derive(Deserialize)]
struct ReadRequest { path: String }

#[derive(Serialize)]
struct SystemStats { cpu_usage: f32, memory_used: u64, status: String }

struct AppState { client: reqwest::Client, sys: Mutex<System> }

// --- HELPERS ---
fn scan_workspace() -> String {
    let mut tree = String::from("PROJECT FILES:\n");
    if let Ok(entries) = fs::read_dir("workspace") {
        for entry in entries.flatten() {
            if let Ok(name) = entry.file_name().into_string() { tree.push_str(&format!("- {}\n", name)); }
        }
    }
    tree
}

async fn run_command_internal(cmd: &str) -> String {
    let output = Command::new("sh").arg("-c").arg(cmd).output();
    match output {
        Ok(out) => format!("{}\n{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr)),
        Err(e) => format!("EXEC_ERROR: {}", e),
    }
}

// --- HANDLERS ---
async fn serve_ui() -> Html<&'static str> { Html(include_str!("../index.html")) }
async fn read_file(Json(req): Json<ReadRequest>) -> String { fs::read_to_string(&req.path).unwrap_or("ERROR".into()) }
async fn run_command(Json(req): Json<CmdRequest>) -> String { run_command_internal(&req.cmd).await }

async fn get_stats(State(state): State<Arc<AppState>>) -> Json<SystemStats> {
    let mut sys = state.sys.lock().unwrap();
    sys.refresh_cpu();
    sys.refresh_memory();
    let usage = sys.global_cpu_info().cpu_usage();
    let status = if usage > 90.0 { "OVERLOAD" } else { "ONLINE" };
    Json(SystemStats {
        cpu_usage: usage,
        memory_used: sys.used_memory() / 1024 / 1024,
        status: status.to_string(),
    })
}

async fn ask_stream(State(state): State<Arc<AppState>>, body: String) -> Sse<impl futures_util::Stream<Item = Result<Event, Infallible>>> {
    let req = match serde_json::from_str::<AskRequest>(&body) {
        Ok(json) => json,
        Err(_) => AskRequest { prompt: body, editor_context: None },
    };

    let client = state.client.clone();
    let project_tree = scan_workspace();
    let initial_context = format!("{}\n\n{}\n\nTASK: {}", SYSTEM_INSTRUCTION, project_tree, req.prompt);

    let stream = async_stream::stream! {
        let api_key = std::env::var("GEMINI_API_KEY").unwrap_or_default();
        
        // FIXED URL CONSTRUCTION:
        // Removed hardcoded "models/" because it is now inside the GEMINI_MODEL constant.
        let url = format!("https://generativelanguage.googleapis.com/{}/{}:streamGenerateContent?alt=sse&key={}", API_VERSION, GEMINI_MODEL, api_key);

        let mut parts = vec![serde_json::json!({ "text": initial_context })];
        if let Some(code) = req.editor_context {
            if !code.is_empty() { parts.push(serde_json::json!({ "text": format!("CURRENT EDITOR:\n```\n{}\n```", code) })); }
        }

        let res_result = client.post(&url).json(&serde_json::json!({ "contents": [{ "parts": parts.clone() }] })).send().await;

        match res_result {
            Ok(res) => {
                let mut byte_stream = res.bytes_stream();
                let mut full_response = String::new();

                while let Some(Ok(bytes)) = byte_stream.next().await {
                    if let Ok(text) = String::from_utf8(bytes.to_vec()) {
                        for line in text.lines().filter(|l| l.starts_with("data: ")) {
                            let content = line.replace("data: ", "");
                            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                                if let Some(chunk) = json["candidates"][0]["content"]["parts"][0]["text"].as_str() {
                                    full_response.push_str(chunk);
                                    yield Ok(Event::default().data(chunk.to_string()));
                                }
                            }
                        }
                    }
                }

                let trimmed = full_response.trim();

                // --- AUTO-LINT & SAVE LOGIC ---
                if let Some(start) = trimmed.find("[SAVE:") {
                    if let Some(end_tag) = trimmed[start..].find("]") {
                        let filename = trimmed[start+6..start+end_tag].trim();
                        if let Some(content_start) = trimmed[start+end_tag..].find("]") {
                             let actual_content_start = start + end_tag + 1;
                             
                             if let Some(save_end) = trimmed[actual_content_start..].find("[/SAVE]") {
                                 let raw_content = &trimmed[actual_content_start..actual_content_start + save_end];
                                 
                                 // Strip Markdown Code Fences
                                 let mut clean_content = raw_content.trim();
                                 if clean_content.starts_with("```") {
                                     if let Some(newline) = clean_content.find('\n') {
                                         clean_content = &clean_content[newline+1..];
                                     }
                                 }
                                 clean_content = clean_content.trim_end_matches("```").trim();

                                 let file_path = format!("workspace/{}", filename);
                                 if let Ok(_) = fs::write(&file_path, clean_content) {
                                     yield Ok(Event::default().data(format!("\n\n>> SYSTEM: File '{}' saved (Cleaned & Linted).", filename)));
                                 }
                             }
                        }
                    }
                }

                if let Some(start) = trimmed.find("[EXEC:") {
                    if let Some(end) = trimmed[start..].find("]") {
                        let cmd = trimmed[start+6..start+end].trim();
                        yield Ok(Event::default().data(format!("\n>> SYSTEM: Executing '{}'...", cmd)));
                        let output = run_command_internal(cmd).await;
                        yield Ok(Event::default().data(format!("\nOUTPUT:\n{}", output)));
                    }
                }
                
                if let Some(start) = trimmed.find("[READ:") {
                    if let Some(end) = trimmed[start..].find("]") {
                        let filename = trimmed[start+6..start+end].trim();
                        let file_path = format!("workspace/{}", filename);
                        match fs::read_to_string(&file_path) {
                            Ok(c) => { yield Ok(Event::default().data(format!("\n\n>> SYSTEM: Read '{}' ({} chars).\n", filename, c.len()))); },
                            Err(_) => { yield Ok(Event::default().data(format!("\n\n>> SYSTEM: Error reading '{}'.", filename))); }
                        }
                    }
                }
            }
            Err(_) => { yield Ok(Event::default().data("ERROR: Neural Link Failed")); }
        }
    };
    Sse::new(stream)
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let _ = fs::create_dir_all("workspace");
    let sys = System::new_all();
    let client = reqwest::Client::new();
    let app_state = Arc::new(AppState { client, sys: Mutex::new(sys) });

    let app = Router::new()
        .route("/", get(serve_ui))
        .route("/api/stats", get(get_stats))
        .route("/ask", post(ask_stream))
        .route("/run_command", post(run_command))
        .route("/read_file", post(read_file))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!(">> NEXUS AGENT ACTIVE (Port 8080)");
    axum::serve(listener, app).await.unwrap();
}
