use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize)]
struct GeminiRequest { contents: Vec<Content> }
#[derive(Serialize)]
struct Content { parts: Vec<Part> }
#[derive(Serialize)]
struct Part { text: String }

#[derive(Deserialize)]
struct GeminiResponse { candidates: Option<Vec<Candidate>> }
#[derive(Deserialize)]
struct Candidate { content: Option<ResponseContent> }
#[derive(Deserialize)]
struct ResponseContent { parts: Option<Vec<ResponsePart>> }
#[derive(Deserialize)]
struct ResponsePart { text: String }

pub struct GemmaEngine {
    client: Client,
    api_key: String,
}

impl GemmaEngine {
    pub fn new() -> Result<Self> {
        // If no key is found, we don't crash. We just warn.
        let api_key = env::var("GEMINI_API_KEY").unwrap_or_else(|_| "MISSING".to_string());
        let client = Client::builder().timeout(std::time::Duration::from_secs(30)).build()?;
        Ok(Self { client, api_key })
    }

    pub async fn generate(&self, prompt: &str) -> Result<String> {
        if self.api_key == "MISSING" {
            return Ok(">> [ERR] API KEY MISSING. Please export GEMINI_API_KEY.".to_string());
        }

        let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}", self.api_key);
        
        let body = GeminiRequest {
            contents: vec![Content { parts: vec![Part { text: prompt.to_string() }] }]
        };

        let res = self.client.post(&url).json(&body).send().await?;
        
        if !res.status().is_success() {
            return Ok(format!(">> [API ERROR] Status Code: {}", res.status()));
        }

        let data: GeminiResponse = res.json().await?;
        
        // Extract the text safely
        let text = data.candidates.as_ref()
            .and_then(|c| c.first())
            .and_then(|c| c.content.as_ref())
            .and_then(|c| c.parts.as_ref())
            .and_then(|p| p.first())
            .map(|p| p.text.clone())
            .unwrap_or_else(|| ">> [ERR] EMPTY RESPONSE FROM GOOGLE".to_string());

        Ok(text)
    }
}
