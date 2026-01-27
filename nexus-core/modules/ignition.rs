use std::process::Command;
pub fn clear_hardware_path() {
    println!(">> IGNITION: Clearing Port 8080...");
    let _ = Command::new("fuser").arg("-k").arg("8080/tcp").output();
}
