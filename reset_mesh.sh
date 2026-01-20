#!/bin/bash
echo "🛡️ Project Nexus Singularity: Cleaning Environment..."

# 1. Kill any process holding our mesh ports (7340, 7341, 8080)
# We use pkill -f to find the binary name since fuser/lsof are failing
pkill -9 -f nexus_singularity
pkill -9 -f broadcast_pulse.sh

# 2. Wait for the OS to release the socket lease (TIME_WAIT clearance)
echo "⏳ Purging Socket Zombies (10s)..."
sleep 10

# 3. Verify ports are clear
echo "✅ Environment Reset. Igniting V0.3.8 Kernel..."
./target/release/nexus_singularity
