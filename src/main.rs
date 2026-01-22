mod heartbeat_beacon;
mod routes;

use std::net::SocketAddr;
use std::thread;
use axum::{routing::get_service, Router};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    println!(">> [NEXUS] BOOTING SOVEREIGN CORE v0.5.0 (TERMINAL ACTIVE)");

    thread::spawn(|| { heartbeat_beacon::start_beacon(); });

    let app = Router::new()
        .nest("/api", routes::files::router())     // File System
        .nest("/api/term", routes::terminal::router()) // New Terminal System
        .nest_service("/", get_service(ServeDir::new("./dist"))); // UI

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!(">> [RETINA] HUD seated at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
