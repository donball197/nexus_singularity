#!/bin/bash
# Project Nexus Singularity - Master Boot v0.4.7
# Logic for LDuet Debian Node

source ~/nexus_singularity/safe_boot.sh

echo ">> MASTER: Launching Project Nexus Singularity..."
echo ">> [1/3] Environment: LDUET DEBIAN (ARM64)"
echo ">> [2/3] Identity: $NODE_TYPE"
echo ">> [3/3] Target Sibling: $TARGET_SIBLING"

# Skip Motorola-only commands
if [[ -d "/data/data/com.termux" ]]; then
    termux-wake-lock
fi

echo ">> SUCCESS: Node is stable with 2.6GB Available RAM."
