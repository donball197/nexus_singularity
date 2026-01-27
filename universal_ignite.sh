#!/bin/bash
# Project Nexus Singularity v0.2.2 - Universal Ignition
# Strategy: Zero Fragmentation Environment Detection

echo "[NEXUS] Detecting Shared DNA..."

# 1. Environment Detection (Motorola vs Chromebook)
if [ -d "/data/data/com.termux" ]; then
    NODE_TYPE="Motorola_ARM64"
    LIB_PATH="./lib/termux"
else
    NODE_TYPE="Chromebook_ARM64"
    LIB_PATH="./lib/crostini"
fi

echo "[NEXUS] Node identified as: $NODE_TYPE"
echo "[NEXUS] Seating libraries from: $LIB_PATH"

# 2. Atomic Kernel Check
if [ ! -f "target/release/nexus_singularity" ]; then
    echo "[!] Atomic Kernel not found. Run 'cargo build --release -j 1' first."
    exit 1
fi

# 3. Ignition Handshake
echo "[NEXUS] Locking Frequency at 528Hz..."
# Execute the logic with specific library seating
LD_LIBRARY_PATH=$LIB_PATH ./target/release/nexus_singularity
