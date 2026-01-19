use std::net::UdpSocket;

pub fn start_pulse_listener() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:7340")?;
    println!("📡 Frequency Ear Active on Port 7340...");
    
    let mut buf = [0; 1024];
    loop {
        let (amt, src) = socket.recv_from(&mut buf)?;
        let msg = String::from_utf8_lossy(&buf[..amt]);
        println!("⚡ Pulse Received from {}: {}", src, msg);
    }
}
