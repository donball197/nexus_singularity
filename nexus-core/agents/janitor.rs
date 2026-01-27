use std::fs;
use std::path::Path;

pub async fn run_janitor() -> String {
    // Ensure archive folder exists
    let _ = fs::create_dir_all("archive");

    // Clean incremental builds
    let _ = fs::remove_dir_all("target/release/incremental");

    // Move old logs & snapshots to archive
    if let Ok(entries) = fs::read_dir("./") {
        for entry in entries.flatten() {
            if let Some(name) = entry.file_name().to_str() {
                if (name.ends_with(".log") || name.contains("snapshot")) && name != "archive" {
                    let _ = fs::rename(entry.path(), Path::new("archive").join(name));
                }
            }
        }
    }

    "✅ Janitor Clean Complete".to_string()
}
