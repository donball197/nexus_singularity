use sysinfo::System;

pub struct NexusMonitor {
    pub sys: System,
}

impl NexusMonitor {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        Self { sys }
    }

    pub fn get_vitals(&mut self) -> String {
        self.sys.refresh_cpu_usage();
        self.sys.refresh_memory();
        
        let cpus = self.sys.cpus();
        let cpu_use: f32 = cpus.iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / cpus.len() as f32;
        let mem_use = self.sys.used_memory() as f32 / self.sys.total_memory() as f32 * 100.0;
        
        format!("CPU: {:.1}% | RAM: {:.1}%", cpu_use, mem_use)
    }
}

// Bridging the call from main.rs
pub fn start_monitor() {
    let mut monitor = NexusMonitor::new();
    println!("[SYSTEM] Monitor seated: {}", monitor.get_vitals());
}
