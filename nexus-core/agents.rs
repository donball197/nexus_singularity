use serde::{Serialize, Deserialize};
use crate::SovereignState;
use std::sync::Arc;

#[derive(Serialize, Deserialize, Clone)]
pub struct MicroAgent {
    pub name: String,
    pub model: String,
    pub task_directive: String,
    pub token_budget: u32,
}

pub async fn spawn_sibling(state: Arc<SovereignState>, agent: MicroAgent) {
    let mut stats = state.stats.lock().await;
    let spawn_msg = format!("HIVE_EXPANSION: Spawning [{}] via {}", agent.name, agent.model);
    stats.status = spawn_msg.clone();
    
    // Log the birth of the sibling in the Neural Memory
    crate::memory::commit_to_journal("COMMANDER", "AGENT_SPAWN", &spawn_msg).await;
}
