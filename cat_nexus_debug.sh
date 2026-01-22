#!/usr/bin/env bash
set -e

echo "===== Cargo.toml ====="
cat Cargo.toml || echo "(missing)"
echo

echo "===== src/main.rs ====="
cat src/main.rs || echo "(missing)"
echo

echo "===== src/routes/mod.rs ====="
cat src/routes/mod.rs || echo "(missing)"
echo

echo "===== src/routes/files.rs ====="
cat src/routes/files.rs || echo "(missing)"
echo

echo "===== src/App.svelte ====="
cat src/App.svelte || echo "(missing)"
echo

echo "===== dist/index.html ====="
cat dist/index.html || echo "(missing)"
echo

echo "===== package.json ====="
cat package.json || echo "(missing)"
echo

echo "===== vite.config.js ====="
cat vite.config.js || echo "(missing)"
echo

echo "===== BUILD (cargo build --release) ====="
cargo build --release || echo "(build failed)"
echo

echo "===== RUN OUTPUT (first 20 lines) ====="
./target/release/nexus_singularity | head -n 20 || echo "(run failed)"
