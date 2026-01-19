use axum::{extract::{State, Json, Query}, response::{IntoResponse, Html, sse::{Event, Sse}}, routing::{get, post}, Router};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use std::process::Command as SysCommand;
use serde_json::json;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use std::collections::HashMap;

mod frequency;
mod brain;
mod action_handler;

#[derive(Deserialize)]
struct SignalPayload { value: u8 }

#[derive(Serialize, Clone)]
pub struct LocalStats {
    pub battery: String,
    pub status: String,
    pub last_command: String,
    pub is_stealth: bool,
}

pub struct SovereignState {
    pub pulses: Mutex<Vec<u8>>,
    pub last_pulse: Mutex<Instant>,
    pub stats: Mutex<LocalStats>,
    pub client: reqwest::Client,
}

impl SovereignState {
    pub async fn vibrate(state: &Arc<SovereignState>, pattern: &str) {
        let stealth = { state.stats.lock().await.is_stealth };
        let tactical_pattern = if stealth {
            pattern.replace("500", "50").replace("200", "30").replace("150", "20")
        } else {
            pattern.to_string()
        };
        let _ = SysCommand::new("termux-vibrate").arg("-p").arg(tactical_pattern).spawn();
    }
}

#[tokio::main]
async fn main() {
    let shared_state = Arc::new(SovereignState {
        pulses: Mutex::new(Vec::new()),
        last_pulse: Mutex::new(Instant::now()),
        stats: Mutex::new(LocalStats {
            battery: "FIELD_INIT".into(),
            status: "Sovereign_Link_Active".into(),
            last_command: "NODE_COLD".into(),
            is_stealth: false,
        }),
        client: reqwest::Client::new(),
    });

    let listener_state = shared_state.clone();
    tokio::spawn(async move { crate::frequency::start_frequency_listener(listener_state).await; });

    let heartbeat_state = shared_state.clone();
    tokio::spawn(async move {
        let node_id = std::env::var("NODE_ID").unwrap_or_else(|_| "FIELD_01".into());
        loop {
            let status = { heartbeat_state.stats.lock().await.status.clone() };
            crate::frequency::broadcast_heartbeat(&node_id, &status).await;
            tokio::time::sleep(Duration::from_secs(15)).await;
        }
    });

    let app = Router::new()
        .route("/", get(handle_index))
        .route("/stats", get(get_local_stats))
        .route("/signal", post(handle_signal))
        .route("/think", get(handle_think))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("🔥 FIELD_NODE: KERNEL_V0.3.8_LIVE");
    axum::serve(listener, app).await.unwrap();
}

async fn handle_think(State(state): State<Arc<SovereignState>>, Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    let prompt = params.get("prompt").cloned().unwrap_or_else(|| "Tactical Update".to_string());
    Sse::new(brain::process_stream(state, prompt))
}

async fn handle_index() -> Html<String> {
    Html(std::fs::read_to_string("index.html").unwrap_or_else(|_| "HUD_MISSING".into()))
}

async fn get_local_stats(State(state): State<Arc<SovereignState>>) -> Json<LocalStats> {
    Json(state.stats.lock().await.clone())
}

async fn handle_signal(State(state): State<Arc<SovereignState>>, Json(payload): Json<SignalPayload>) -> impl IntoResponse {
    let mut pulses = state.pulses.lock().await;
    let mut last_time = state.last_pulse.lock().await;
    let mut stats = state.stats.lock().await;
    let now = Instant::now();
    SovereignState::vibrate(&state, "30").await;

    if now.duration_since(*last_time) > Duration::from_millis(1200) && !pulses.is_empty() {
        let mut morse = String::new();
        for &p in &*pulses { morse.push(if p == 1 { '.' } else { '-' }); }
        let cmd = match morse.as_str() {
            "....." => "1", "....-" => "2", "...--" => "3", "-...." => "6",
            _ => "0",
        };
        execute_sovereign_command(cmd, &mut stats, &state).await;
        pulses.clear();
    }
    pulses.push(payload.value);
    *last_time = now;
    axum::http::StatusCode::OK
}

async fn execute_sovereign_command(cmd: &str, stats: &mut LocalStats, state: &Arc<SovereignState>) {
    let api_key = std::env::var("GEMINI_API_KEY").unwrap_or_default();
    let node_id = std::env::var("NODE_ID").unwrap_or_else(|_| "FIELD_01".into());

    match cmd {
        "1" => {
            if let Ok(o) = SysCommand::new("termux-battery-status").output() {
                stats.battery = String::from_utf8_lossy(&o.stdout).trim().to_string();
            }
            stats.status = "Telemetry_Updated".into();
        }
        "3" => {
            let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemma-3-27b-it:generateContent?key={api_key}");
            let body = json!({"contents": [{"parts": [{"text": "Field status report. 5 words. ALL CAPS."}]}]});
            if let Ok(res) = state.client.post(&url).json(&body).send().await {
                if let Ok(j) = res.json::<serde_json::Value>().await {
                    if let Some(t) = j["candidates"][0]["content"]["parts"][0]["text"].as_str() {
                        let result = t.trim().to_uppercase();
                        stats.status = format!("FIELD_GEMMA: {}", result);
                        crate::frequency::broadcast_result(&node_id, &result).await;
                    }
                }
            }
        }
        "6" => {
            let _ = SysCommand::new("termux-camera-photo").arg("-c").arg("0").arg("snapshot.jpg").output();
            if let Ok(img) = std::fs::read("snapshot.jpg") {
                let b64 = BASE64.encode(img);
                crate::frequency::broadcast_snapshot(&node_id, &b64).await;
                stats.status = "📸 TRANSMITTED".into();
            }
        }
        _ => {}
    }
    stats.last_command = format!("CMD_{}", cmd);
}
