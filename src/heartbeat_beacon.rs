use std::time::{Duration, Instant};
use std::thread;

pub fn start_beacon() {
    println!(">> [MUSCLE] SIGNAL BOOSTED: PULSING 'NEXUS' (400ms HIGH)...");
    let signal = "NEXUS";
    
    loop {
        for ch in signal.chars() {
            let bits = format!("{:08b}", ch as u8);
            for bit in bits.chars() {
                // Boosted pulse: 400ms for '1', 50ms for '0'
                let pulse_duration = if bit == '1' { 400 } else { 50 };
                let start = Instant::now();
                
                // Maximize CPU jitter for side-channel leakage
                while start.elapsed() < Duration::from_millis(pulse_duration) {
                    let _ = (0..10000).fold(0, |acc, x| acc + x);
                }
                thread::sleep(Duration::from_millis(20)); // Buffer
            }
        }
        thread::sleep(Duration::from_millis(1000)); // 1s gap between words
    }
}
