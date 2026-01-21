#!/bin/bash
# Project Nexus Singularity - Sovereign Launch Protocol
source ~/nexus_singularity/safe_boot.sh

echo ">> [PULSE] Launching Nexus Singularity GUI..."
echo ">> [RAM] Applying WebKit Memory Guard..."

# Disabling compositing mode to save 150-300MB of RAM on ARM64
WEBKIT_DISABLE_COMPOSITING_MODE=1 ~/nexus_singularity/target/release/nexus_singularity
