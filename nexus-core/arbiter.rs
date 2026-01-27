use crate::brain::AgentDNA;

pub struct TaskArbiter {
    pub active_agents: Vec<String>,
}

impl TaskArbiter {
    pub fn negotiate_cycles(&self, dna: &AgentDNA) {
        // High Intuition weight (0.95) allows agents to 'pre-empt' low-logic tasks
        if dna.intuition_weight > 0.90 {
            println!("[ARBITER] Intuitive override: Prioritizing Neural Inference.");
        }
    }
}
