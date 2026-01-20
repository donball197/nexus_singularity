#!/bin/bash
# Project Nexus Singularity: Universal Ignite v0.2.2
# Strategy: Environment Detection & Dynamic Library Seating

echo "🛰️ INITIALIZING NEXUS SINGULARITY DNA..."

# 1. Environment Detection (Motorola vs. Chromebook)
if [ -d "/data/data/com.termux" ]; then
    NODE_ID="FIELD_01_MOTO"
    ORT_DIR="/root/ort_lib_backup/onnxruntime-linux-aarch64-1.19.0/lib"
    echo "NODE: $NODE_ID detected (Termux/Android Architecture)"
else
    NODE_ID="BASE_01_DUET"
    ORT_DIR="$HOME/ort_lib/onnxruntime-linux-aarch64-1.19.0/lib"
    echo "NODE: $NODE_ID detected (Crostini/Linux Architecture)"
fi

# 2. Seating the Muscles (ONNX Runtime)
export ONNXRUNTIME_LIB_PATH=$ORT_DIR
export LD_LIBRARY_PATH=$ONNXRUNTIME_LIB_PATH:$LD_LIBRARY_PATH
echo "MUSCLES SEATED: $LD_LIBRARY_PATH"

# 3. Modular UI Sync
echo "SYNCING MODULAR FACE..."
mkdir -p ~/nexus_singularity/src-tauri/static
cp ~/nexus_singularity/static/index.html ~/nexus_singularity/src-tauri/static/index.html

# 4. Hardware Health Check (Dependencies)
if ! pkg-config --exists gdk-3.0; then
    echo "⚠️ MISSING GDK-3.0 MUSCLES. RUN: apt install libgtk-3-dev"
fi

# 5. Ignite the Brain
echo "IGNITING THE BRAIN ON $NODE_ID..."
cd ~/nexus_singularity/src-tauri
ORT_STRATEGY=system cargo tauri build -- -j 1

echo "🚀 BUILD COMPLETE. DNA: STABLE."
