use std::fs::File;
use std::io::Write;
use futures_util::StreamExt;
use reqwest::Client;

pub async fn download_bitnet_model() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    // The "Powerhouse" BitNet 1.58 GGUF (Optimized for 2GB)
    let url = "https://huggingface.co/1-bit-LLM/bitnet_b1_58-3B/resolve/main/bitnet_b1_58-3B-Q2_K.gguf";
    
    println!(">> [ARCHITECT] COMMENCING STREAMED BRUTE-FORCE DOWNLOAD...");
    println!(">> [TARGET] /dev/shm/nexus_model.gguf");

    let response = client.get(url).send().await?;
    let mut dest = File::create("/dev/shm/nexus_model.gguf")?;
    let mut stream = response.bytes_stream();

    let mut downloaded: u64 = 0;

    while let Some(item) = stream.next().await {
        let chunk = item?;
        dest.write_all(&chunk)?;
        downloaded += chunk.len() as u64;
        
        // Visual Pulse every 10MB
        if downloaded % (10 * 1024 * 1024) < chunk.len() as u64 {
            print!("\r>> [PULSE] {} MB INGESTED...", downloaded / (1024 * 1024));
            std::io::stdout().flush()?;
        }
    }

    println!("\n>> [ARCHITECT] BITNET MODEL SECURED IN VMO.");
    Ok(())
}
