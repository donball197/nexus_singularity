pub struct RevenueStats {
    pub local_tasks: u32,
    pub cloud_tasks: u32,
}

impl RevenueStats {
    pub fn new() -> Self {
        Self { local_tasks: 0, cloud_tasks: 0 }
    }

    pub fn log_task(&mut self, is_cloud: bool) {
        if is_cloud { self.cloud_tasks += 1; }
        else { self.local_tasks += 1; }
    }

    pub fn display_vitals(&self) {
        println!("--- REVENUE VITALS ---");
        println!("Local (Free): {}", self.local_tasks);
        println!("Cloud (Cost): {}", self.cloud_tasks);
        println!("----------------------");
    }
}
