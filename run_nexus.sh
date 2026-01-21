#!/bin/bash
# Project Nexus Singularity - Execution v0.4.8

echo ">> MASTER: Launching Sovereign UI..."

# Ensure Caffeine is active
if command -v caffeine >/dev/null 2>&1; then
    caffeine -a &
fi

# Launch Tauri
cd ~/nexus_singularity
cargo tauri dev
