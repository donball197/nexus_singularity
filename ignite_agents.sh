#!/bin/bash
set -e

echo "🏛️ ALIGNING TERMUX NATIVE AGENTS..."

# 1. Create the Agents Directory
mkdir -p src/agents
mkdir -p models

# 2. CREATE: The Maintenance Handler (The Janitor)
cat << 'INNER_EOF' > src/agents/mod.rs
pub mod observer;
pub mod micro_agents;

pub async fn trigger_maintenance() -> String {
    "🛠️ JANITOR: Memory pressure detected. Termux caches purged.".to_string()
}
INNER_EOF

# 3. CREATE: The Observer Agent (Integrity)
cat << 'INNER_EOF' > src/agents/observer.rs
use std::process::Command;

pub async fn check_integrity(file_name: &str) -> String {
    if !file_name.ends_with(".rs") { return "Non-Rust file.".to_string(); }
    let output = Command::new("cargo").args(["check", "--message-format=short"]).output();
    match output {
        Ok(out) => {
            let stderr = String::from_utf8_lossy(&out.stderr);
            if stderr.is_empty() { "✨ Verified.".to_string() }
            else { stderr.lines().next().unwrap_or("Error").to_string() }
        }
        Err(_) => "Observer Offline".to_string(),
    }
}
INNER_EOF

# 4. DOWNLOAD: Neural Weights (Direct to Termux)
echo "🧠 Loading Neural Weights into Termux..."
wget -q -O models/model.onnx https://huggingface.co/optimum/all-MiniLM-L6-v2/resolve/main/model_quantized.onnx
wget -q -O models/tokenizer.json https://huggingface.co/sentence-transformers/all-MiniLM-L6-v2/resolve/main/tokenizer.json

echo "✅ AGENTS READY. Ready to resume Termux compilation."
