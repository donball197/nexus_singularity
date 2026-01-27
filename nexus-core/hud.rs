use std::os::unix::net::UnixStream;
use std::io::{Read, stdout, Write};
use std::thread;
use std::time::Duration;

fn main() -> std::io::Result<()> {
    println!("\x1b[93m[ SOVEREIGN HUD ] STATUS: NEURAL_LINK_ACTIVE\x1b[0m");
    let mut stream = match UnixStream::connect("NEXUS.SOCK") {
        Ok(s) => s,
        Err(_) => {
            println!(">> [ERROR] NEXUS.SOCK NOT FOUND.");
            return Ok(());
        }
    };
    let mut buffer = [0; 512];
    loop {
        if let Ok(n) = stream.read(&mut buffer) {
            if n > 0 {
                print!("\r\x1b[32m>> [PULSE]\x1b[0m {}          ", String::from_utf8_lossy(&buffer[..n]).trim());
                let _ = stdout().flush();
            }
        }
        thread::sleep(Duration::from_millis(10));
    }
}
