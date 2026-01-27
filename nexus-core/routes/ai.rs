use axum::{routing::post, extract::Json, Router};
use serde::{Deserialize, Serialize};
#[derive(Deserialize)]
pub struct PromptRequest { pub prompt: String }
#[derive(Serialize)]
pub struct PromptResponse { pub response: String, pub status: String }
pub fn router() -> Router { Router::new().route("/ask", post(handle_prompt)) }
async fn handle_prompt(Json(req): Json<PromptRequest>) -> Json<PromptResponse> {
    Json(PromptResponse {
        response: format!("Gemma 3 received: {}", req.prompt),
        status: "success".to_string()
    })
}
