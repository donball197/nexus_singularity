use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;
use std::env;

fn main() {
    let home = env::var("HOME").unwrap();
    let config_path = format!("{}/nexus_singularity/agents/dev_01/config.json", home);
    let stats_path = format!("{}/nexus_singularity/agents/dev_01/stats.csv", home);

    if !Path::new(&config_path).exists() { std::process::exit(1); }
    let mut config_file = File::open(config_path).unwrap();
    let mut config_str = String::new();
    config_file.read_to_string(&mut config_str).unwrap();
    
    let threshold: f32 = config_str.split("\"threshold\":")
        .collect::<Vec<&str>>().get(1)
        .map(|s| s.split(',').collect::<Vec<&str>>()[0].trim())
        .unwrap_or("1.5")
        .parse()
        .unwrap_or(1.5);

    if !Path::new(&stats_path).exists() { std::process::exit(0); }
    let file = File::open(stats_path).unwrap();
    let reader = BufReader::new(file);
    if let Some(Ok(last_line)) = reader.lines().last() {
        let parts: Vec<&str> = last_line.split(',').collect();
        if parts.len() >= 2 {
            let load_val: f32 = parts[1].trim().parse().unwrap_or(0.0);
            if load_val > threshold {
                println!("!! [ALERT] LOAD {} > THRESHOLD {} !!", load_val, threshold);
                std::process::exit(2);
            }
        }
    }
    std::process::exit(0);
}
