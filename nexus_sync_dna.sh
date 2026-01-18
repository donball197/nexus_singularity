#!/bin/bash
echo "--- BEGIN NEXUS DNA EXFILTRATION ---"

echo "FILE: Cargo.toml"
cat ~/nexus_singularity/Cargo.toml || echo "NOT_FOUND"
echo "------------------------"

echo "FILE: src/main.rs"
cat ~/nexus_singularity/src/main.rs || echo "NOT_FOUND"
echo "------------------------"

echo "FILE: src/brain.rs"
cat ~/nexus_singularity/src/brain.rs || echo "NOT_FOUND"
echo "------------------------"

echo "FILE: src/server.rs"
cat ~/nexus_singularity/src/server.rs || echo "NOT_FOUND"
echo "------------------------"

echo "FILE: src/action_handler.rs"
cat ~/nexus_singularity/src/action_handler.rs || echo "NOT_FOUND"
echo "------------------------"

echo "FILE: index.html"
cat ~/nexus_singularity/index.html || echo "NOT_FOUND"
echo "------------------------"

echo "DIRECTORY_MAP: models"
ls -R ~/nexus_singularity/models || echo "EMPTY"
echo "------------------------"

echo "ENV_VARS: LD_LIBRARY_PATH"
echo $LD_LIBRARY_PATH
echo "------------------------"

echo "--- END NEXUS DNA EXFILTRATION ---"
