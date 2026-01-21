#!/bin/bash
source ~/nexus_singularity/safe_boot.sh
echo ">> MASTER: Launching Ecosystem..."
if [[ "$NODE_TYPE" == "LENOVO_COMMAND" ]]; then
    command -v caffeine >/dev/null && caffeine -a &
fi
echo ">> SUCCESS: Project Nexus Singularity Online."
