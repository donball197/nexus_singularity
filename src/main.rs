mod modules;
use modules::telemetry;
use modules::{ignition, engine, sensory, security};
use std::sync::atomic::{AtomicUsize, AtomicBool, Ordering};
use crate::micro_agents::MicroAgent;
use axum::{routing::get, response::{Html, Redirect}, Router};
use std::net::SocketAddr;
use std::fs::{OpenOptions, read_to_string, remove_file};
use std::io::Write;
use std::process::Command;
mod micro_agents;
mod monitor;

use once_cell::sync::Lazy;
pub static MONITOR: Lazy<monitor::CapabilityMonitor> = Lazy::new(monitor::CapabilityMonitor::new);
pub static CURRENT_DELAY: AtomicUsize = AtomicUsize::new(250);
pub static STRESS_ACTIVE: AtomicBool = AtomicBool::new(false);
pub static FILTER_ACTIVE: AtomicBool = AtomicBool::new(true);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // SEATING THE SPARK
    ignition::clear_hardware_path();

    let app = Router::new()
        .route("/", get(|| async { 
            let is_stress = STRESS_ACTIVE.load(Ordering::SeqCst);
            let bg_base = if is_stress { "#900" } else { "#112" };
            Html(format!("<html><body style='background:{}; color:#0f0;'><h2>NEXUS MODULAR KERNEL</h2><a href='/stress'>TOGGLE STRESS</a></body></html>", bg_base))
        }))
        .route("/stress", get(|| async {
            let current = STRESS_ACTIVE.load(Ordering::SeqCst);
            STRESS_ACTIVE.store(!current, Ordering::SeqCst);
            Redirect::to("/")
        }));

    tokio::spawn(async move {
        let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
        if let Ok(listener) = tokio::net::TcpListener::bind(addr).await {
            axum::serve(listener, app).await.unwrap();
        }
    });

    let mut agent = MicroAgent::init()?;
    println!(">> SUCCESS: Multi-Micro AI Kernel Recovered.");

    loop {
        // CALLING THE ENGINE BLOCK
        let mut ids = vec![0i64; 128];
        if let Some(logits) = engine::process_math_handshake(&mut agent, ids) {
            // Processing logic...
        }
        tokio::time::sleep(std::time::Duration::from_millis(CURRENT_DELAY.load(Ordering::SeqCst) as u64)).await;
    }
}
