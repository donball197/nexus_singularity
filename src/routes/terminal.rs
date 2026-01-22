use axum::{
    routing::post,
    Router,
    Json,
    response::IntoResponse,
};
use serde::Deserialize;
use std::process::Command;

#[derive(Deserialize)]
struct CmdPayload {
    cmd: String,
}

// The 'Voice' Function: Speaks to the Kernel
async fn run_cmd(Json(p): Json<CmdPayload>) -> impl IntoResponse {
    println!(">> [EXEC] {}", p.cmd); // Log to the server console
    let output = Command::new("sh")
        .arg("-c")
        .arg(&p.cmd)
        .output();
        
    match output {
        Ok(o) => {
            let stdout = String::from_utf8_lossy(&o.stdout).to_string();
            let stderr = String::from_utf8_lossy(&o.stderr).to_string();
            format!("{}{}", stdout, stderr)
        }
        Err(e) => format!("Execution Failed: {}", e),
    }
}

pub fn router() -> Router {
    Router::new().route("/exec", post(run_cmd))
}
