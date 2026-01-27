use std::io::{self, Write};

pub struct TerminalCore {
    history: Vec<String>,
}

impl TerminalCore {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self { history: Vec::new() })
    }

    pub fn run(&mut self, engine: &crate::gemma_engine::GemmaEngine) -> anyhow::Result<()> {
        println!("\x1b[1;34m[HUD]\x1b[0m Chronos Memory Module Engaged.");
        
        loop {
            print!("\x1b[1;32m>> COMMANDER\x1b[0m ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let cmd = input.trim();

            if cmd == "exit" || cmd == "quit" {
                break;
            } else if cmd == "status" {
                println!("\x1b[1;36m[TELEMETRY]\x1b[0m History Depth: {} exchanges", self.history.len() / 2);
            } else if !cmd.is_empty() {
                // Construct the context from history
                let context = self.history.join("\n");
                let full_prompt = format!("{}\nUser: {}\nAssistant:", context, cmd);

                match engine.process_thought(&full_prompt) {
                    Ok(response) => {
                        println!("\x1b[1;35m[GEMMA]\x1b[0m {}", response);
                        // Store the exchange (limit to last 10 lines to save tokens)
                        self.history.push(format!("User: {}", cmd));
                        self.history.push(format!("Assistant: {}", response));
                        if self.history.len() > 10 { self.history.remove(0); }
                    },
                    Err(e) => println!("\x1b[1;31m[ERROR]\x1b[0m {}", e),
                }
            }
        }
        Ok(())
    }
}
