use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use portable_pty::{native_pty_system, PtySize, CommandBuilder};

pub struct SovereignTerminal {
    pub writer: Box<dyn Write + Send>,
    pub output_buffer: Arc<Mutex<Vec<String>>>,
}

impl SovereignTerminal {
    pub fn new() -> Self {
        let pty_system = native_pty_system();
        // Correcting open_pty to openpty for v0.8.1
        let pair = pty_system.openpty(PtySize {
            rows: 24, cols: 80, pixel_width: 0, pixel_height: 0,
        }).expect("PTY Open Failed");

        let cmd = CommandBuilder::new("bash");
        let _child = pair.slave.spawn_command(cmd).expect("Bash Spawn Failed");
        
        let mut reader = pair.master.try_clone_reader().expect("Reader Clone Failed");
        let buffer = Arc::new(Mutex::new(Vec::new()));
        let buffer_clone = Arc::clone(&buffer);

        std::thread::spawn(move || {
            let mut buf = [0u8; 1024];
            while let Ok(n) = reader.read(&mut buf) {
                if n == 0 { break; }
                let mut lines = buffer_clone.lock().unwrap();
                let content = String::from_utf8_lossy(&buf[..n]);
                for line in content.lines() {
                    lines.push(line.to_string());
                    if lines.len() > 100 { lines.remove(0); }
                }
            }
        });

        Self {
            writer: pair.master.take_writer().expect("Writer Take Failed"),
            output_buffer: buffer,
        }
    }

    pub fn write(&mut self, input: &str) {
        let _ = self.writer.write_all(input.as_bytes());
        let _ = self.writer.flush();
    }
}
