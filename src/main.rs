mod frequency;
mod action_handler;
mod web_bridge;
mod action_listener;

use axum::{extract::State, routing::get, Json, Router};
use std::sync::Arc;
use web_bridge::WebBridge;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("🛸 Project Nexus Singularity V0.3.9 Booting...");

    let bridge = Arc::new(WebBridge::new());
    let bridge_for_listener = Arc::clone(&bridge);
    let bridge_for_web = Arc::clone(&bridge);

    // 📡 Background Task: Pulse Listener (Feeds the Bridge)
    tokio::spawn(async move {
        if let Err(e) = frequency::start_pulse_listener(bridge_for_listener) {
            eprintln!("❌ Pulse Listener Error: {}", e);
        }
    });

    // 🛡️ Background Task: Action Listener (Port 7341)
    tokio::spawn(async move {
        if let Err(e) = action_listener::start_action_listener() {
            eprintln!("❌ Action Listener Error: {}", e);
        }
    });

    // 🌐 Web Server Configuration
    let app = Router::new()
        .route("/api/mesh", get(get_mesh_status))
        .with_state(bridge_for_web);

    let addr = "0.0.0.0:8080";
    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("🏛️ Web Bridge API LIVE at http://localhost:8080");

    axum::serve(listener, app).await
}

async fn get_mesh_status(State(bridge): State<Arc<WebBridge>>) -> Json<serde_json::Value> {
    let state = bridge.state.lock().unwrap();
    Json(serde_json::json!({
        "node": "FIELD_01",
        "last_pulse": state.last_pulse_from,
        "count": state.heartbeat_count,
        "status": state.status,
        "version": "V0.3.9"
    }))
}
