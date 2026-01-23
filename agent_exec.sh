#!/bin/bash
# USAGE: ./agent_exec.sh <agent_name> <filename_or_instruction>
AGENT=$1
INPUT=$2
AGENT_ROOT="$HOME/nexus_singularity/agents/$AGENT"
TARGET="$AGENT_ROOT/$INPUT"

if [ ! -d "$AGENT_ROOT" ]; then
    echo ">> [ERR] AGENT NOT FOUND: $AGENT"
    exit 1
fi

# CRITICAL: Switch context to the agent's folder so imports/builds work
cd "$AGENT_ROOT" || exit 1

echo ">> [SYSTEM] PROCESSING REQUEST ON PROCESSOR 0..."
echo "------------------------------------------------"

# 1. CARGO CHECK (If user asks to run "cargo")
if [ "$INPUT" == "cargo" ] && [ -f "Cargo.toml" ]; then
    cargo run
    echo "------------------------------------------------"
    exit 0
fi

if [ ! -f "$INPUT" ]; then
    echo ">> [ERR] 404: FILE NOT FOUND: $INPUT"
    exit 1
fi

# 2. EXECUTION LOGIC
case "$INPUT" in
    *.py) python3 "$INPUT" 2>&1 ;;
    *.js) node "$INPUT" 2>&1 ;;
    *.sh) bash "$INPUT" 2>&1 ;;
    *.rs)
        # Transient compile-and-run
        if rustc "$INPUT" -o "temp_run" 2>&1; then
            ./temp_run
            rm ./temp_run
        else
            echo ">> [ERR] RUST COMPILATION FAILED. SEE LOGS ABOVE."
        fi
        ;;
    *.cpp|*.cc)
        if g++ "$INPUT" -o "temp_run" 2>&1; then
            ./temp_run
            rm ./temp_run
        else
            echo ">> [ERR] C++ COMPILATION FAILED. SEE LOGS ABOVE."
        fi
        ;;
    *) echo ">> [ERR] UNKNOWN FILETYPE." ;;
esac

echo "------------------------------------------------"
