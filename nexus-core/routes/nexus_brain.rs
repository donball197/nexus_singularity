use axum::{Json, response::IntoResponse};
use serde::{Deserialize, Serialize};
use std::env;
use std::process::Command;
use reqwest::Client;
use serde_json::Value;

#[derive(Deserialize)]
pub struct Prompt { pub text: String }

#[derive(Serialize)]
struct GeminiRequest { contents: Vec<Content> }
#[derive(Serialize)]
struct Content { parts: Vec<Part> }
#[derive(Serialize)]
struct Part { text: String }

pub async fn ask_gemini(Json(p): Json<Prompt>) -> impl IntoResponse {
    let api_key = env::var("GEMINI_API_KEY").unwrap_or("MISSING".to_string());
    if api_key == "MISSING" { return "Error: API Key missing.".to_string(); }

    let client = Client::new();
    let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemma-3-27b-it:generateContent?key={}", api_key);
    
    let system = "You are THE COMMANDER (Gemma 3). You control the Nexus server.
    If the user asks for a sub-agent, output: 'ACTION: SPAWN <agent_name>'.
    Otherwise, answer concisely.";

    let body = GeminiRequest {
        contents: vec![Content {
            parts: vec![Part { text: format!("{}\n\nUSER: {}", system, p.text) }]
        }]
    };

    let res = client.post(&url).json(&body).send().await;

    match res {
        Ok(r) => {
            let raw_json = r.text().await.unwrap_or("{}".to_string());
            
            // --- PARSE THE JSON ---
            let v: Value = serde_json::from_str(&raw_json).unwrap_or(Value::Null);
            
            // Extract just the spoken text
            let mut answer = match v["candidates"][0]["content"]["parts"][0]["text"].as_str() {
                Some(text) => text.to_string(),
                None => return format!("Raw Data (Parse Error): {}", raw_json)
            };

            // --- THE SOVEREIGN REFLEX ---
            if answer.contains("ACTION: SPAWN") {
                let parts: Vec<&str> = answer.split("ACTION: SPAWN").collect();
                if let Some(suffix) = parts.get(1) {
                     let agent_name = suffix.trim().split_whitespace().next().unwrap_or("agent_x");
                     let clean_name = agent_name.replace("\"", "").replace("\\n", "");

                     let output = Command::new("bash")
                        .arg("-c")
                        .arg(format!("~/nexus_singularity/spawn_sibling.sh {}", clean_name))
                        .output();

                     match output {
                         Ok(o) => {
                             let logs = String::from_utf8_lossy(&o.stdout);
                             answer.push_str(&format!("\n\n>> [SYSTEM LOG]\n{}", logs));
                         },
                         Err(e) => answer.push_str(&format!("\n\n>> [SYSTEM ERR] Failed: {}", e))
                     }
                }
            }
            
            answer
        },
        Err(e) => format!("Connection Error: {}", e)
    }
}
