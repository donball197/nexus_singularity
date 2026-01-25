use std::sync::Arc;
mod gemma_engine;
mod system_monitor;
mod terminal_core;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("--- PROJECT NEXUS SINGULARITY v0.2.2 ---");
    println!("[SYSTEM] Shared DNA detected. Aligning Lazarus OS Spine...");

    let args: Vec<String> = std::env::args().collect();
    let is_tui = args.iter().any(|arg| arg == "--mode" || arg == "tui");

    // Corrected: Using init() as defined in gemma_engine.rs
    let engine = Arc::new(gemma_engine::GemmaEngine::init()?);
    println!("[NEURAL] Gemma 3 Intelligence seated in the Spine.");

    if is_tui {
        println!("[SYSTEM] Entering TUI Mode...");
        // This keeps the cockpit open
        terminal_core::run_terminal(engine).await?;
    } else {
        println!("[UPLINK] Bridge active at http://127.0.0.1:3000");
        crate::system_monitor::start_monitor();
        // This keeps the background engine alive until Ctrl+C
        tokio::signal::ctrl_c().await?;
    }

    Ok(())
}
