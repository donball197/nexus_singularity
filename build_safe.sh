#!/bin/bash
source ~/nexus_singularity/safe_boot.sh
echo ">> FORGE: Clearing Stale Locks..."
cargo clean
export CARGO_INCREMENTAL=0
echo ">> FORGE: Commencing Restricted Build (-j 1)..."
cargo build --release -j 1
