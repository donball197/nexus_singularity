use std::process::Command;
use std::fs;

pub async fn check_integrity(file_path: &str) -> String {
    // Only check if it's a Rust file to save Moto G battery/RAM
    if !file_path.endsWith(".rs") { return "Ready".to_string(); }

    let output = Command::new("cargo")
        .args(["check", "--color", "never"])
        .output();

    match output {
        Ok(out) => {
            let stderr = String::from_utf8_lossy(&out.stderr);
            if stderr.contains("error") {
                stderr.lines().take(3).collect::<Vec<_>>().join("\n")
            } else {
                "✨ Integrity Verified".to_string()
            }
        },
        Err(_) => "Observer Offline".to_string(),
    }
}
