use std::sync::Mutex;
use once_cell::sync::Lazy;

static LOG_BUFFER: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(Vec::new()));

pub fn add_log(entry: String) {
    let mut logs = LOG_BUFFER.lock().unwrap();
    logs.push(entry);
    if logs.len() > 10 { logs.remove(0); } // Keep last 10
}

pub fn show_logs() {
    println!("\n\x1b[1m[HIVE SUBCONSCIOUS - RECENT LOGS]\x1b[0m");
    println!("----------------------------------");
    let logs = LOG_BUFFER.lock().unwrap();
    if logs.is_empty() {
        println!("No logs recorded. Run a prompt first.");
    } else {
        for (i, log) in logs.iter().enumerate() {
            println!("{}. {}", i + 1, log);
        }
    }
    println!("----------------------------------\n");
}

pub fn purge_logs() {
    let mut logs = LOG_BUFFER.lock().unwrap();
    logs.clear();
}
