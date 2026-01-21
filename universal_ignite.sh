#!/bin/bash
# Project Nexus Singularity - Universal Ignite v0.3.9
# Detects Environment and seats the correct Sibling/Muscles.

if [[ -d "/data/data/com.termux" ]]; then
    export NODE_TYPE="MOTOROLA_EDGE"
    export TARGET_SIBLING=$EDGE_AGENT_MODEL
    echo "Detected: Motorola (Android/Termux) - Seating Edge Agent."
else
    export NODE_TYPE="LENOVO_COMMAND"
    export TARGET_SIBLING=$COMMANDER_MODEL
    echo "Detected: Lenovo (Linux/Crostini) - Seating Commander."
fi

# Finalize Environment
source ~/.env_nexus
echo "Node Identity: $NODE_TYPE | Active Sibling: $TARGET_SIBLING"
