use axum::{routing::get, Router, response::Html};
use std::net::SocketAddr;
use crate::MONITOR;
use std::sync::atomic::Ordering;

pub async fn start_ui() {
    let app = Router::new().route("/", get(handler));
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("🌐 Dashboard Live at: http://localhost:8080");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<String> {
    let total = MONITOR.total_invocations.load(Ordering::SeqCst);
    Html(format!(
        "<html>
            <head>
                <style>
                    body {{ background: #020617; color: #38bdf8; font-family: monospace; display: flex; justify-content: center; align-items: center; height: 100vh; margin: 0; }}
                    .card {{ border: 2px double #38bdf8; padding: 2rem; border-radius: 8px; background: #0f172a; text-align: center; box-shadow: 0 0 20px #0ea5e9; }}
                    h1 {{ color: #f8fafc; border-bottom: 1px solid #334155; padding-bottom: 10px; }}
                    .status {{ font-size: 1.5rem; color: #22c55e; margin: 10px 0; }}
                </style>
                <script>setTimeout(() => location.reload(), 2000);</script>
            </head>
            <body>
                <div class='card'>
                    <h1>NEXUS HUD</h1>
                    <div class='status'>PLUMB & LEVEL</div>
                    <div>Invocations: {}</div>
                </div>
            </body>
        </html>",
        total
    ))
}
