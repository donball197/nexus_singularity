use std::net::UdpSocket;
use std::sync::Arc;
use crate::web_bridge::WebBridge;

pub fn start_pulse_listener(bridge: Arc<WebBridge>) -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:7340")?;
    println!("📡 Frequency Ear Active on Port 7340...");
    
    let mut buf = [0; 1024];
    loop {
        let (amt, src) = socket.recv_from(&mut buf)?;
        let sender_ip = src.ip().to_string();
        
        // Feed the data to the UI Bridge
        bridge.update_state(&sender_ip);
        
        println!("⚡ Pulse Logged from {}", sender_ip);
    }
}
