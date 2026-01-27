use reqwest::Client;
use serde_json::json;

pub struct GeminiAPI {
    client: Client,
    key: String,
}

impl GeminiAPI {
    pub fn init() -> Result<Self, anyhow::Error> {
        // Using your established environment variable
        let key = std::env::var("GEMINI_API_KEY")?;
        Ok(Self { client: Client::new(), key })
    }

    pub async fn request_inference(&self, prompt: &str) -> Result<String, anyhow::Error> {
        let url = "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent";
        
        let response = self.client.post(format!("{}?key={}", url, self.key))
            .json(&json!({
                "contents": [{
                    "parts": [{"text": prompt}]
                }],
                "generationConfig": {
                    "temperature": 0.7,
                    "maxOutputTokens": 1024
                }
            }))
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;
        
        let text = response["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .unwrap_or("Inference Error")
            .to_string();
            
        Ok(text)
    }
}
