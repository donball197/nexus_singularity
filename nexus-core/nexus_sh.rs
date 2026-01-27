use dotenvy::dotenv;
mod vitals_module;
mod nexus_bridge;
mod logs_module;
use std::io::{self, Write};

#[tokio::main]
async fn main() {
    dotenv().ok();
    println!("\x1b[32m[SYSTEM] Shell Active. API Link: ESTABLISHED.\x1b[0m");
    loop {
        print!("\x1b[1;36mNEXUS > \x1b[0m");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() { break; }
        let cmd = input.trim();
        if cmd.is_empty() { continue; }

        match cmd {
            "clear" => { print!("{esc}[2J{esc}[1;1H", esc = 27 as char); continue; },
            "vitals" => vitals_module::show_vitals(),
            "logs" => logs_module::show_logs(),
            "purge" => { logs_module::purge_logs(); println!("[SYSTEM] Logs cleared."); continue; },
            "exit" | "quit" => {
                println!("[SYSTEM] Shutting down Hive...");
                break;
            },
            _ => {
                println!("\x1b[35m[AGENT SCAN]\x1b[0m Contacting Gemma API...");
                match nexus_bridge::call_gemma(cmd).await {
                    Ok(response) => {
                        println!("\n\x1b[32m[RESPONSE]\x1b[0m\n{}\n", response);
                        logs_module::add_log(format!("Prompt: \"{}\" | Status: SUCCESS", cmd));
                    },
                    Err(e) => {
                        println!("\x1b[31m[ERROR]\x1b[0m API Stutter: {}", e);
                        logs_module::add_log(format!("Prompt: \"{}\" | Status: ERROR ({})", cmd, e));
                    }
                }
            }
        }
    }
}
