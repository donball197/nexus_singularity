use reqwest::Client;
use serde_json::{json, Value};
use std::env;

pub async fn call_gemma(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY not set");
    let client = Client::new();
    
    // Explicit 2026 Production Model: Gemma 3 27B Instruction-Tuned
    let model = "gemma-3-27b-it"; 
    let url = format!("https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}", model, api_key);

    let payload = json!({
        "contents": [{
            "parts": [{
                "text": prompt
            }]
        }]
    });

    let res = client.post(url)
        .json(&payload)
        .send()
        .await?
        .json::<Value>()
        .await?;

    // Deep-dive parsing for Gemma 3 candidates
    let text = res["candidates"][0]["content"]["parts"][0]["text"].as_str();

    match text {
        Some(t) => Ok(t.to_string()),
        None => {
            let error_msg = res["error"]["message"].as_str().unwrap_or("Unknown Stutter");
            Ok(format!("\x1b[31m[GEMMA 3 ERROR]\x1b[0m {}", error_msg))
        }
    }
}
