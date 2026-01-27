use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SovereignState {
    pub logic_weight: f32,
    pub intuition_weight: f32,
    pub axiomatic_state: String,
}

pub struct AgentDNA {
    pub state: SovereignState,
}

impl AgentDNA {
    pub fn load_memory() -> SovereignState {
        SovereignState {
            logic_weight: 0.85,
            intuition_weight: 0.95,
            axiomatic_state: "INIT_STABLE".to_string(),
        }
    }

    pub fn update_axioms(&mut self, new_state: &str) {
        self.state.axiomatic_state = new_state.to_string();
    }
}
