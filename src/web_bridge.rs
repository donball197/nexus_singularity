use std::sync::{Arc, Mutex};
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct MeshState {
    pub last_pulse_from: String,
    pub heartbeat_count: u64,
    pub status: String,
}

pub struct WebBridge {
    pub state: Arc<Mutex<MeshState>>,
}

impl WebBridge {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(MeshState {
                last_pulse_from: "None".to_string(),
                heartbeat_count: 0,
                status: "COLD".to_string(),
            })),
        }
    }

    pub fn update_state(&self, sender_ip: &str) {
        let mut state = self.state.lock().unwrap();
        state.last_pulse_from = sender_ip.to_string();
        state.heartbeat_count += 1;
        state.status = "WARM".to_string();
    }
}
