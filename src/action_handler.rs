use std::process::Command;
use std::sync::Arc;
use crate::AppState;
use std::io::Write;

pub async fn git_auto_commit(filename: &str) -> String {
    let _ = Command::new("git").arg("init").status();
    let _ = Command::new("git").arg("add").arg(filename).status();
    let msg = format!("Auto: {} at {}", filename, chrono::Local::now().format("%H:%M:%S"));
    if Command::new("git").args(["commit", "-m", &msg]).status().is_ok() {
        format!("✅ Committed: {}", filename)
    } else { "⚠️ No changes".to_string() }
}

pub async fn git_revert_file(filename: &str) -> String {
    if Command::new("git").args(["checkout", "HEAD^", "--", filename]).status().is_ok() {
        format!("⏪ Reverted: {}", filename)
    } else { "❌ Revert Failed".to_string() }
}

pub fn parse_action(resp: &str) -> Option<String> {
    if resp.contains("[SHELL]") && resp.contains("[/SHELL]") {
        let start = resp.find("[SHELL]").unwrap() + 7;
        let end = resp.find("[/SHELL]").unwrap();
        Some(resp[start..end].to_string())
    } else { None }
}

pub async fn execute_directive(state: Arc<AppState>, cmd: String) -> String {
    let mut stdin = state.debian_stdin.lock().unwrap();
    let _ = writeln!(stdin, "{}", cmd);
    let _ = stdin.flush();
    format!("Executed: {}", cmd)
}
