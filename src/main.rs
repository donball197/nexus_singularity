use axum::Router;
use tower_http::services::ServeDir;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Seating the Eyes (Svelte build folder)
    let serve_dir = ServeDir::new("build")
        .fallback(ServeDir::new("build/index.html"));

    let app = Router::new()
        .fallback_service(serve_dir);

    let addr = "0.0.0.0:8080";
    let listener = TcpListener::bind(addr).await.unwrap();
    
    println!(">> [OPTICS] Sovereign UI LIVE at http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}
