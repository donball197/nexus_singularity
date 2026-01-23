#!/bin/bash
# Code Fusion MDK v5.6 - Sovereign Runner
# Preserving build: nexus_singularity

echo "--- FUSION IGNITION ---"
# Cleanup locks and stale sockets
find . -name "*.lock" -type f -delete
rm -f /tmp/nexus.sock

# Dynamic Environment Seating
export SOCK_PATH="/tmp/nexus.sock"
export RUST_LOG=info

echo "Target: nexus_singularity"
echo "Socket: $SOCK_PATH"
echo "-----------------------"

# Execute the confirmed bin target
cargo run --bin nexus_singularity
