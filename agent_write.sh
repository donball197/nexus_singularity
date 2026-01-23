#!/bin/bash
# USAGE: ./agent_write.sh <agent_name> <filename> <base64_content>
AGENT=$1
FILENAME=$2
B64_CONTENT=$3

# Define target paths
AGENT_ROOT="$HOME/nexus_singularity/agents/$AGENT"
TARGET="$AGENT_ROOT/$FILENAME"

# Ensure directory exists
mkdir -p "$(dirname "$TARGET")"

# Decode Base64 directly into the file
# This bypasses all shell quoting issues
echo "$B64_CONTENT" | base64 -d > "$TARGET"

echo ">> [SYSTEM] Wrote $(wc -c < "$TARGET") bytes to agents/$AGENT/$FILENAME"
