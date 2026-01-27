use std::fs::{File};
use std::io::{Write, BufRead, BufReader};
use std::os::unix::net::UnixStream;
use std::env;

fn main() {
    let home = env::var("HOME").unwrap();
    let socket_path = format!("{}/nexus_singularity/data/nexus.sock", home);

    let mut mem_available = 0.0;
    if let Ok(file) = File::open("/proc/meminfo") {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(l) = line {
                if l.starts_with("MemAvailable:") {
                    let parts: Vec<&str> = l.split_whitespace().collect();
                    if parts.len() >= 2 {
                        mem_available = parts[1].parse::<f64>().unwrap_or(0.0) / 1024.0 / 1024.0;
                    }
                    break;
                }
            }
        }
    }

    if let Ok(mut stream) = UnixStream::connect(&socket_path) {
        let _ = stream.write_all(format!("{:.2}GB", mem_available).as_bytes());
    }
    println!(">> [RAM] Sent {:.2}GB to Bridge", mem_available);
}
