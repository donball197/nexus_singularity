mod heartbeat_beacon;
mod routes;
use axum::{routing::get_service, Router};
use tower_http::services::ServeDir;
#[tokio::main]
async fn main() {
    println!(">> [NEXUS] BOOTING v0.5.1");
    std::thread::spawn(|| { heartbeat_beacon::start_beacon(); });
    let app = Router::new()
        .nest("/api/files", routes::files::router())
        .nest("/api/term", routes::terminal::router())
        .nest("/api/ai", routes::ai::router())
        .nest_service("/", get_service(ServeDir::new("./dist")));
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
