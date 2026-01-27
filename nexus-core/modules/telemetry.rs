use std::fs::OpenOptions;
use std::io::Write;
use chrono::Local;

pub fn log_performance(tps: f64, stability: f64, device_name: &str) {
    let mut file = OpenOptions::new().create(true).append(true).open("performance.log").unwrap();
    writeln!(file, "[{}] DEVICE: {} | TPS: {:.2} | STABILITY: {:.2}%", 
        Local::now().format("%Y-%m-%d %H:%M:%S"), device_name, tps, stability).unwrap();
}
