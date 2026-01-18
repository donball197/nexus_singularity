use crate::micro_agents::MicroAgent;
use crate::MONITOR;
use std::sync::atomic::Ordering;

pub fn process_math_handshake(agent: &mut MicroAgent, input: Vec<i64>) -> Option<Vec<f32>> {
    if let Ok(logits) = agent.prepare_and_run(input, 128) {
        MONITOR.active_tokens.fetch_add(128, Ordering::SeqCst);
        return Some(logits);
    }
    None
}
