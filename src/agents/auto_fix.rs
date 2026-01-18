use std::process::Command;

pub async fn run_diagnostic_build() -> Result<String, String> {
    let output = Command::new("cargo")
        .args(["check", "--message-format=short"])
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok("✅ Diagnostic Build: Clean".to_string())
    } else {
        let err_msg = String::from_utf8_lossy(&output.stderr).to_string();
        Err(format!("❌ Diagnostic Build Errors:\n{}", err_msg))
    }
}
