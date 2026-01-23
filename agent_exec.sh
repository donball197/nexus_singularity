#!/bin/bash
# USAGE: ./agent_exec.sh <agent_name> <filename>

AGENT=$1
FILENAME=$2
ROOT_DIR="$HOME/nexus_singularity"
TARGET="$ROOT_DIR/agents/$AGENT/$FILENAME"

if [ ! -f "$TARGET" ]; then 
    echo ">> [ERR] TARGET NOT FOUND: $TARGET"
    exit 1
fi

EXT="${FILENAME##*.}"
case $EXT in
    rs)
        # Compile if missing or if source is newer than binary
        if [ ! -f "${TARGET%.*}" ] || [ "$TARGET" -nt "${TARGET%.*}" ]; then
            rustc "$TARGET" -o "${TARGET%.*}"
        fi
        "${TARGET%.*}"
        ;;
    py)
        # Use venv python if active, otherwise fallback to system
        if [ -f "$ROOT_DIR/venv/bin/python3" ]; then
            "$ROOT_DIR/venv/bin/python3" "$TARGET"
        else
            python3 "$TARGET"
        fi
        ;;
esac

EXIT_CODE=$?
exit $EXIT_CODE
