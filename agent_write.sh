#!/bin/bash
# USAGE: ./agent_write.sh <agent_name> <filename> <base64_content>
AGENT=$1
FILENAME=$2
B64_CONTENT=$3

TARGET="$HOME/nexus_singularity/agents/$AGENT/$FILENAME"
mkdir -p "$(dirname "$TARGET")"

# Decode Base64 directly into the file (bypassing shell quote issues)
echo "$B64_CONTENT" | base64 -d > "$TARGET"

echo ">> [SYSTEM] Wrote $(wc -c < "$TARGET") bytes to agents/$AGENT/$FILENAME"
