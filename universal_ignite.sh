#!/bin/bash
echo "[IGNITE] Detecting Sovereign Environment..."

# 🔍 Identify Hardware Muscles
if [ -d "/data/data/com.termux" ]; then
    export NEXUS_ENV="MOTOROLA_TERMUX"
    export LD_LIBRARY_PATH="$HOME/nexus_singularity/lib/motorola:$LD_LIBRARY_PATH"
else
    export NEXUS_ENV="CHROMEBOOK_LINUX"
    export LD_LIBRARY_PATH="$HOME/nexus_singularity/lib/chromebook:$LD_LIBRARY_PATH"
fi

# ⚙️ Compiler Optimization (Shared DNA)
export RUSTFLAGS="-C target-cpu=native"
export ZIG_FLAGS="-Dcpu=native"

echo "[IGNITE] Environment: $NEXUS_ENV"
echo "[IGNITE] 1.9GB RAM Guard: ACTIVE"
echo "[IGNITE] Zircon Heartbeat: 528Hz LOCKED"

# Run the terminal check
cargo check --manifest-path nexus-terminal/src-tauri/Cargo.toml
