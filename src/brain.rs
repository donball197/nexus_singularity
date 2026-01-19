use crate::SovereignState;
use crate::action_handler;
use axum::response::sse::Event;
use futures_util::{Stream, StreamExt};
use serde_json::json;
use std::sync::Arc;
use std::convert::Infallible;

pub fn process_stream(
    state: Arc<SovereignState>,
    prompt: String,
) -> impl Stream<Item = Result<Event, Infallible>> + Send {
    async_stream::stream! {
        let api_key = std::env::var("GEMINI_API_KEY").unwrap_or_default();
        let model = "gemma-3-27b-it"; // Hardcoded for Motorola stability
        
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:streamGenerateContent?alt=sse&key={}",
            model, api_key
        );

        let sys_msg = "You are FIELD_AGENT GEMMA-3. Brief tactical output. Use [SHELL]command[/SHELL] for local action.";
        let body = json!({"contents": [{"parts": [{"text": format!("{}\nUSER: {}", sys_msg, prompt)}]}]});

        if let Ok(res) = state.client.post(&url).json(&body).send().await {
            let mut byte_stream = res.bytes_stream();
            let mut full_resp = String::new();

            while let Some(result) = byte_stream.next().await {
                if let Ok(chunk) = result {
                    let text = String::from_utf8_lossy(&chunk);
                    for line in text.lines().filter(|l| l.starts_with("data: ")) {
                        let data = &line[6..].trim();
                        if *data == "[DONE]" { break; }
                        if let Ok(j) = serde_json::from_str::<serde_json::Value>(data) {
                            if let Some(token) = j["candidates"][0]["content"]["parts"][0]["text"].as_str() {
                                full_resp.push_str(token);
                                yield Ok(Event::default().data(token));
                            }
                        }
                    }
                }
            }
            if let Some(cmd) = action_handler::parse_action(&full_resp) {
                let out = action_handler::execute_directive(state.clone(), cmd).await;
                yield Ok(Event::default().data(format!("\n[FIELD_ACTION]: {}", out)));
            }
        }
    }
}
