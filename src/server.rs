use axum::{
    extract::{Path, State},
    response::{sse::{Event, Sse, KeepAlive}, IntoResponse},
    Json,
};
use serde::{Deserialize, Serialize};
use std::{fs, sync::Arc, convert::Infallible, io::Write};
use tokio::sync::Mutex;
use futures_util::stream;
use sysinfo::System;
use chrono::Utc;
use crate::micro_agents::MicroAgent;

pub struct AppState {
    pub micro_agent: Arc<Mutex<MicroAgent>>,
    pub sys: Arc<Mutex<System>>,
}

#[derive(Deserialize, Serialize)]
pub struct PromptRequest { pub prompt: String }

#[derive(Serialize)]
pub struct SessionLog {
    pub timestamp: String,
    pub prompt: String,
    pub response: String,
}

pub async fn handle_prompt(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<PromptRequest>,
) -> impl IntoResponse {
    let mut agent = state.micro_agent.lock().await;
    let _ = agent.prepare_and_run(vec![1, 0], 2);
    
    let response_text = format!("GUARDIAN Core: Handshake with '{}' verified.", payload.prompt);

    let log = SessionLog {
        timestamp: Utc::now().to_rfc3339(),
        prompt: payload.prompt.clone(),
        response: response_text.clone(),
    };
    
    let _ = fs::create_dir_all("workspace/sessions");
    let filename = format!("workspace/sessions/log_{}.json", Utc::now().timestamp_millis());
    if let Ok(mut file) = fs::File::create(filename) {
        let _ = file.write_all(serde_json::to_string_pretty(&log).unwrap().as_bytes());
    }

    let stream = stream::iter(
        response_text
            .split_whitespace()
            .map(|word| Ok::<Event, Infallible>(Event::default().data(format!("{} ", word))))
            .collect::<Vec<_>>()
    );

    Sse::new(stream).keep_alive(KeepAlive::default())
}

#[derive(Serialize)]
pub struct FileList { pub files: Vec<String> }

pub async fn list_files() -> Json<FileList> {
    let mut files = vec![];
    if let Ok(entries) = fs::read_dir("workspace/sessions") {
        for entry in entries.flatten() {
            if let Ok(name) = entry.file_name().into_string() { files.push(format!("sessions/{}", name)); }
        }
    }
    Json(FileList { files })
}

pub async fn read_file(Path(file): Path<String>) -> impl IntoResponse {
    let sanitized = file.replace("..", "");
    fs::read_to_string(format!("workspace/{}", sanitized)).unwrap_or_else(|_| "File access error".to_string())
}

#[derive(Serialize)]
pub struct SystemStats {
    pub cpu_usage: f32,
    pub ram_used: u64,
    pub ram_total: u64,
}

pub async fn get_stats(State(state): State<Arc<AppState>>) -> Json<SystemStats> {
    let mut sys = state.sys.lock().await;
    sys.refresh_all();
    Json(SystemStats {
        cpu_usage: sys.global_cpu_info().cpu_usage(),
        ram_used: sys.used_memory() / 1024 / 1024,
        ram_total: sys.total_memory() / 1024 / 1024,
    })
}
