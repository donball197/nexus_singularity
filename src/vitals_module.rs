use std::fs;

pub fn show_vitals() {
    // Read actual memory stats from the Linux Kernel
    let meminfo = fs::read_to_string("/proc/meminfo").unwrap_or_default();
    let total = meminfo.lines().find(|l| l.starts_with("MemTotal")).unwrap_or("0").split_whitespace().nth(1).unwrap_or("1").parse::<f64>().unwrap_or(1.0);
    let available = meminfo.lines().find(|l| l.starts_with("MemAvailable")).unwrap_or("0").split_whitespace().nth(1).unwrap_or("0").parse::<f64>().unwrap_or(0.0);
    
    let usage_pct = (1.0 - (available / total)) * 100.0;

    println!("\n\x1b[1m[HIVE RESONANCE STATUS - LIVE SILICON]\x1b[0m");
    println!("----------------------------------");
    for i in 0..68 {
        let color = if usage_pct < 70.0 { "\x1b[32m" } else if usage_pct < 90.0 { "\x1b[33m" } else { "\x1b[31m" };
        print!("{}■\x1b[0m ", color);
        if (i + 1) % 17 == 0 { println!(""); }
    }
    println!("\n\x1b[1;36m>> TOTAL: 68 NODES | RAM USAGE: {:.1}% | PULSE: 528Hz\x1b[0m", usage_pct);
    println!("----------------------------------\n");
}
