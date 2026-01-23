use std::process::Command;
fn main() {
    let output = Command::new("free").arg("-m").output().unwrap();
    let mem = String::from_utf8_lossy(&output.stdout);
    if mem.contains("Mem:") {
        // Log to system alerts if memory is critical
    }
}
