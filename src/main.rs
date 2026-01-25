mod gemma_engine;
mod terminal_core;
mod system_monitor;

use std::sync::Arc;
use crate::gemma_engine::GemmaEngine;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Corrected to use .new() as defined in your engine
    let ai = Arc::new(GemmaEngine::new());
    
    terminal_core::run_terminal(ai).await?;
    Ok(())
}
