use std::env;

pub struct RevenueStats {
    pub local_tasks: u32,
    pub api_calls: u32,
    pub estimated_cost: f64,
}

impl RevenueStats {
    pub fn new() -> Self {
        let behavior = env::var("SYSTEM_BEHAVIOR").unwrap_or_else(|_| "UNKNOWN".to_string());
        println!("Monitor Initialized: {} Mode", behavior);
        
        Self {
            local_tasks: 0,
            api_calls: 0,
            estimated_cost: 0.0,
        }
    }

    pub fn log_task(&mut self, is_api: bool, token_count: u32) {
        if is_api {
            self.api_calls += 1;
            // Est. cost for Gemini 2.5/3 Pro tiers per 1k tokens
            self.estimated_cost += (token_count as f64 / 1000.0) * 0.002; 
        } else {
            self.local_tasks += 1;
        }
    }

    pub fn display_vitals(&self) {
        println!("\n--- NEXUS REVENUE VITALS ---");
        println!("Local Nano Tasks: {}", self.local_tasks);
        println!("Cloud API Calls:  {}", self.api_calls);
        println!("Estimated Burn:   ${:.4}", self.estimated_cost);
        println!("----------------------------");
    }
}
