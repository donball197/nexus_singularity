#!/bin/bash
# NEXUS SINGULARITY v0.2.2 - ENVIRONMENT ALIGNMENT
echo "RESONANCE CHECK: Aligning Infrastructure..."

# Ensure binary vault exists
mkdir -p logs/nexus_vault

# Clean old artifacts for Zero Fragmentation
rm -rf target/*.so
rm -rf logs/nexus_vault/*.lock

# Build the unified engine
cargo build --release

echo "ALIGNMENT COMPLETE. Run './target/release/nexus_singularity' to engage."
