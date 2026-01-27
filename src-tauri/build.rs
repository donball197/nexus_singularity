use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    
    // 1. COMPILE THE ZIG SHIM (The Heartbeat)
    // We use 'zig build-obj' to create the molecular resonance engine
    Command::new("zig")
        .args(&[
            "build-obj", 
            "../src/swap_core.zig", 
            "-femit-bin=swap_core.o", 
            "-fPIC"
        ])
        .current_dir(&Path::new(&out_dir))
        .status()
        .expect(">> ERR: Failed to compile Zig shim!");

    // 2. CREATE THE STATIC LIBRARY
    Command::new("ar")
        .args(&["crus", "libswap_core.a", "swap_core.o"])
        .current_dir(&Path::new(&out_dir))
        .status()
        .expect(">> ERR: Failed to archive Zig library!");

    // 3. WELD TO RUST
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=swap_core");
    
    // 4. LINK ZIRCON HANDLES
    println!("cargo:rustc-link-lib=zircon");

    tauri_build::build();
}
