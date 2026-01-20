#!/bin/bash
# Project Nexus Singularity: Mesh Health Dashboard V0.3.9

echo -e "\033[1;32m📡 Monitoring FIELD_01 (192.168.1.38) Mesh Health...\033[0m"
echo "--------------------------------------------------------"

while true; do
    # Clear line and show current stats
    PULSES=$(grep -c "Sent:" <(timeout 1s cat <(tail -f ~/nexus_singularity/pulse.log 2>/dev/null)))
    DIRECTIVES=$(grep -c "Executing Remote Directive" <(timeout 1s cat <(tail -f ~/nexus_singularity/kernel.log 2>/dev/null)))
    
    echo -ne "\r🚀 Outbound Pulses: $PULSES | 📥 Inbound Directives: $DIRECTIVES | Status: NODE_WARM"
    sleep 2
done
