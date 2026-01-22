#!/bin/bash
# USAGE: ./agent_read.sh <agent_name> <filename>
AGENT=$1
FILENAME=$2
TARGET="$HOME/nexus_singularity/agents/$AGENT/$FILENAME"

if [ -f "$TARGET" ]; then
    echo ">> [SYSTEM] RETRIEVING DATA FROM: $AGENT/$FILENAME"
    echo "------------------------------------------------"
    cat "$TARGET"
    echo -e "\n------------------------------------------------"
else
    echo ">> [ERR] 404: MEMORY FILE NOT FOUND AT $TARGET"
fi
