use std::process::Command;
use std::sync::Arc;
use crate::SovereignState;

pub async fn execute_directive(_state: Arc<SovereignState>, cmd_string: String) -> String {
    let output = if cfg!(target_os = "android") {
        Command::new("sh").arg("-c").arg(&cmd_string).output()
    } else {
        Command::new("bash").arg("-c").arg(&cmd_string).output()
    };

    match output {
        Ok(out) => String::from_utf8_lossy(&out.stdout).to_string(),
        Err(e) => format!("Execution Error: {}", e),
    }
}

pub fn parse_action(text: &str) -> Option<String> {
    if let Some(start) = text.find("[SHELL]") {
        if let Some(end) = text.find("[/SHELL]") {
            return Some(text[start + 7..end].to_string());
        }
    }
    None
}
