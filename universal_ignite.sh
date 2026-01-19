#!/bin/bash
# Project Nexus Singularity: Environment Auto-Detection (V0.3.8)

echo "📡 Initializing Mesh Node..."

# Detect Hardware DNA
MODEL=$(getprop ro.product.model 2>/dev/null)
if [[ "$MODEL" == *"moto"* ]]; then
    NODE_ID="FIELD_01 (Motorola)"
    BUILD_CMD="cargo build"
elif [[ "$MODEL" == *"Duet"* ]] || [[ "$(uname -a)" == *"penguin"* ]]; then
    NODE_ID="COMMAND_DECK (Lenovo Duet)"
    # The 4GB Shield: Force single-threaded build to prevent OOM
    BUILD_CMD="cargo build -j 1"
else
    NODE_ID="UNKNOWN_NODE"
    BUILD_CMD="cargo build"
fi

echo "✅ Detected Node: $NODE_ID"

# Clean artifacts to prevent "Binary Fragmentation"
echo "🧹 Clearing cross-platform artifacts..."
rm -rf target/
rm -f lib/*.so

echo "🚀 Setup Complete. Use '$BUILD_CMD' to compile for this hardware."
