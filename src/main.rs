mod revenue_monitor;
mod interface;
mod network;
mod frequency;
mod action_handler;
pub mod web_bridge;
mod action_listener;

use axum::{extract::State, routing::get, Json, Router};
use std::sync::Arc;
use web_bridge::WebBridge;
use tokio::net::TcpListener;

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
    
    // 🏛️ Sovereign Bridge: Convert std::net to tokio::net
    let std_listener = crate::network::oracle::create_listener(addr).expect("Failed to bind port");
    std_listener.set_nonblocking(true)?;
    let listener = TcpListener::from_std(std_listener)?;
    
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

fn start_ui_thread() {
    std::thread::spawn(|| {
        if let Err(e) = interface::launch_ui() {
            eprintln!("UI Error: {:?}", e);
        }
    });
}
