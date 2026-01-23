use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let output = Command::new("ps").args(&["-eo", "pcpu,comm", "--sort=-pcpu"]).output().unwrap();
    let ps_data = String::from_utf8_lossy(&output.stdout);
    let top_proc: String = ps_data.lines().take(4).collect::<Vec<&str>>().join("\n");
    let path = "/home/donball197/nexus_singularity/agents/dev_01/alerts.log";

    let mut file = OpenOptions::new().append(true).create(true).open(path).unwrap();
    writeln!(file, "--- ANOMALY: {} ---\n{}\n", now, top_proc).unwrap();
    println!(">> [RESPONDER] TRIAGE LOGGED.");
}
