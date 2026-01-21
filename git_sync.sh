#!/bin/bash
echo ">> FORGE: Commencing DNA Sync..."
cd ~/nexus_singularity
git add .
git commit -m "Node Sync: LENOVO_COMMAND | $(date +'%Y-%m-%d %H:%M')"
git push origin main
echo ">> SUCCESS: DNA is synced."
