#!/bin/bash
termux-wake-lock 2>/dev/null
echo ">> [SYSTEM] SOVEREIGN HEARTBEAT ACTIVE"
while true; do
    ~/nexus_singularity/agent_exec.sh dev_01 stats.rs > /dev/null 2>&1
    ~/nexus_singularity/agent_exec.sh dev_01 architect.py > /dev/null 2>&1
    ~/nexus_singularity/agent_exec.sh dev_01 monitor.py
    sleep 60
done
