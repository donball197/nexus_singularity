#!/bin/bash
# Project Nexus Singularity - Safe Boot v0.4.4
# THE "ZERO-SPAZZ" PROTOCOL

echo "--- [SHIELD] INITIALIZING SAFE BOOT ---"

# A. CLEAR GHOSTS: Kill hung AI processes and web ports
pkill -9 -f nexus_singularity 2>/dev/null
pkill -9 -f gemma 2>/dev/null
lsof -ti :8080,7341 | xargs -r kill -9 2>/dev/null

# B. RAM SANITY CHECK: Protect the container from OOM
FREE_RAM=$(free -m | awk '/^Mem:/{print $4}')
if [ "$FREE_RAM" -lt 800 ]; then
    echo "!!! CRITICAL: LOW RAM (${FREE_RAM}MB). FORCING API-ONLY MODE !!!"
    export FORCE_API=true
fi

# C. DNA SYNC: Ensure universal_ignite is seated
if [ -f ~/nexus_singularity/universal_ignite.sh ]; then
    source ~/nexus_singularity/universal_ignite.sh
else
    echo "!!! DNA ERROR: universal_ignite.sh missing !!!"
fi

echo "--- [SHIELD] NODE STABILIZED ---"
