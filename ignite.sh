#!/bin/bash
echo ">> INITIALIZING SOVEREIGN IGNITION..."

# Clear stale Port 8080 locks (Address already in use)
fuser -k 8080/tcp 2>/dev/null

# Sync and optimize build
cargo build --release --quiet

echo ">> SUCCESS: Port 8080 Cleared. Firing Nexus Singularity..."
./target/release/nexus_singularity
