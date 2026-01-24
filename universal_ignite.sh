#!/bin/bash
if [ -d "/data/data/com.termux" ]; then
    ENV="MOTOROLA_TERMUX"
    export LD_LIBRARY_PATH=$HOME/nexus_singularity/lib:$LD_LIBRARY_PATH
else
    ENV="DUET_DEBIAN"
    export LD_LIBRARY_PATH=$HOME/nexus_singularity/lib:/usr/local/lib:$LD_LIBRARY_PATH
fi

echo -e "\x1b[1;35m[IGNITE]\x1b[0m Detected Node: $ENV"

# Stage 1: Kernel
./target/release/nexus_bridge & 
BRIDGE_PID=$!
sleep 2

# Stage 2: Swarm Seating
echo -e "\x1b[1;32m[BOOT]\x1b[0m Seating 68 Agents in RAM..."
sleep 4

# Stage 3: Shell
./target/release/nexus_sh

kill $BRIDGE_PID
