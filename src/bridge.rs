use std::os::unix::net::UnixListener;
use std::io::{Read, Write};
use std::fs;

fn main() -> std::io::Result<()> {
    let socket_path = "NEXUS.SOCK";
    
    // Cleanup old socket DNA
    let _ = fs::remove_file(socket_path);
    
    let listener = UnixListener::bind(socket_path)?;
    println!("\x1b[94m[ BRIDGE ] RAM_SOCKET LIVE: {}\x1b[0m", socket_path);

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = [0; 512];
                if let Ok(n) = stream.read(&mut buffer) {
                    // Broadcast to HUD (Logic seated here)
                    println!(">> [RELAY] {} bytes moved.", n);
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
    Ok(())
}
