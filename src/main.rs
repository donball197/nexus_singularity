mod frequency;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("🛸 Project Nexus Singularity V0.3.8 Booting...");

    // Start the Frequency Listener in a background task
    tokio::spawn(async {
        if let Err(e) = frequency::start_pulse_listener() {
            eprintln!("❌ Pulse Listener Error: {}", e);
        }
    });

    println!("🔒 Kernel Active. Awaiting Handshake...");
    
    // Keep the main thread alive
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await;
    }
}
