use sysinfo::{System, SystemExt, CpuExt};

pub struct Vitals {
    pub cpu: f32,
    pub ram: f32,
}

pub fn get_vitals(sys: &mut System) -> Vitals {
    sys.refresh_all();
    let cpu = sys.global_cpu_info().cpu_usage();
    let ram = (sys.used_memory() as f32 / sys.total_memory() as f32) * 100.0;
    Vitals { cpu, ram }
}

pub fn format_telemetry(vitals: Vitals) -> String {
    format!("[SYS]|{:.1}|{:.1}", vitals.cpu, vitals.ram)
}
