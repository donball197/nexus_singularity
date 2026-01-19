use tokio::net::UdpSocket;
use std::net::SocketAddr;
use serde_json::json;
use std::sync::Arc;
use crate::SovereignState;

pub async fn start_frequency_listener(state: Arc<SovereignState>) {
    let socket = UdpSocket::bind("0.0.0.0:7340").await.expect("Frequency Bind Fail");
    socket.set_broadcast(true).expect("Broadcast Fail");
    let mut buf = [0; 65535]; 

    println!("📡 FIELD_FREQUENCY: Active on Port 7340");

    loop {
        match socket.recv_from(&mut buf).await {
            Ok((len, _addr)) => {
                let message = String::from_utf8_lossy(&buf[..len]);
                if let Ok(incoming) = serde_json::from_str::<serde_json::Value>(&message) {
                    let node_id = incoming["node"].as_str().unwrap_or("Unknown");
                    let current_node = std::env::var("NODE_ID").unwrap_or_else(|_| "FIELD_01".into());

                    if node_id != current_node {
                        let mut stats = state.stats.lock().await;
                        if let Some(res) = incoming["result"].as_str() {
                            stats.status = format!("CMD_DECK: {}", res);
                        }
                    }
                }
            }
            Err(e) => eprintln!("Mesh Error: {}", e),
        }
    }
}

pub async fn broadcast_snapshot(node_id: &str, base64_img: &str) {
    let socket = UdpSocket::bind("0.0.0.0:0").await.unwrap();
    socket.set_broadcast(true).unwrap();
    let target_addr: SocketAddr = "255.255.255.255:7340".parse().unwrap();
    let msg = json!({ "node": node_id, "snapshot": base64_img }).to_string();
    let _ = socket.send_to(msg.as_bytes(), target_addr).await;
}

pub async fn broadcast_result(node_id: &str, result: &str) {
    let socket = UdpSocket::bind("0.0.0.0:0").await.unwrap();
    socket.set_broadcast(true).unwrap();
    let target_addr: SocketAddr = "255.255.255.255:7340".parse().unwrap();
    let msg = json!({ "node": node_id, "result": result }).to_string();
    let _ = socket.send_to(msg.as_bytes(), target_addr).await;
}

pub async fn broadcast_heartbeat(node_id: &str, status: &str) {
    let socket = UdpSocket::bind("0.0.0.0:0").await.unwrap();
    socket.set_broadcast(true).unwrap();
    let target_addr: SocketAddr = "255.255.255.255:7340".parse().unwrap();
    let msg = json!({ "node": node_id, "status": status }).to_string();
    let _ = socket.send_to(msg.as_bytes(), target_addr).await;
}
