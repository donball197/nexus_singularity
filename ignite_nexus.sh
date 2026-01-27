#!/bin/bash
# Project Nexus Singularity: Unified Ignition Protocol
# Targets ARM64 (Motorola/Chromebook) | < 2GB RAM Strategy

echo "[!] Purging device-specific artifacts to prevent ARM64 fragmentation..."
rm -rf ~/lazarus_os/target/ ~/nexus_singularity/target/
rm -f ./lib/*.so

echo "[+] Seating Lazarus OS (Resource Muscles)..."
cd ~/lazarus_os && cargo build --release

echo "[+] Seating Nexus Singularity (Neural Brain)..."
cd ~/nexus_singularity
# Ensure lib directory exists for device-specific .so seating
mkdir -p ./lib
cp ~/lazarus_os/target/release/*.so ./lib/ 2>/dev/null || echo "[!] No .so found, proceeding with static link."

echo "[+] Forging Unified Binary..."
cargo build --release

echo "[+] 528Hz Heartbeat Online. J2 Architect Active."
./target/release/nexus_singularity
