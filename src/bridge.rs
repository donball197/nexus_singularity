use axum::{routing::get, Router, extract::State, response::IntoResponse, extract::ws::{WebSocket, WebSocketUpgrade}};
use std::sync::Arc;
use crate::AppState;

pub fn router() -> Router<Arc<AppState>> {
    // router must explicitly expect Arc<AppState>
    Router::new().route("/ws", get(handle_ws))
}

async fn handle_ws(ws: WebSocketUpgrade, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    // Fix E0521: move the state into the closure, no self reference
    ws.on_upgrade(move |socket| client_loop(socket, state))
}

async fn client_loop(mut socket: WebSocket, _state: Arc<AppState>) {
    use futures_util::StreamExt;
    while let Some(Ok(msg)) = socket.next().await {
        if let Ok(txt) = msg.to_text() {
            let _ = socket.send(axum::extract::ws::Message::Text(format!("Nexus Echo: {}", txt))).await;
        }
    }
}
