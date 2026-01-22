#!/bin/bash
AGENT_NAME=$1

# Safety checks
if [ -z "$AGENT_NAME" ]; then echo "ERR: No agent name provided."; exit 1; fi
ROOT_DIR="$HOME/nexus_singularity"
TARGET_DIR="$ROOT_DIR/agents/$AGENT_NAME"

# Genesis
if [ -d "$TARGET_DIR" ]; then echo "WARN: Agent exists."; exit 0; fi
mkdir -p "$TARGET_DIR"

# Identity
echo "{\"designation\": \"$AGENT_NAME\", \"status\": \"ACTIVE\", \"created\": \"$(date)\"}" > "$TARGET_DIR/identity.json"
echo ">> [SYSTEM] $AGENT_NAME online. Awaiting orders." > "$TARGET_DIR/memory.log"
touch "$TARGET_DIR/draft.txt"

# Signal the Core
echo ">> [KERNEL] SPAWN COMPLETE: $AGENT_NAME"
echo ">> [KERNEL] MEMORY ALLOCATED: 256MB (Virtual)"
