#!/bin/bash
echo ">> [SYSTEM] RUNNING INTEGRITY SCAN..."
ROOT="$HOME/nexus_singularity"
ERRORS=0

# 1. Check Directories
for dir in "agents/dev_01" "agents/memory_manager" "data" "logs"; do
    if [ -d "$ROOT/$dir" ]; then echo "[OK] Directory: $dir"; else echo "[!!] MISSING: $dir"; ERRORS=$((ERRORS+1)); fi
done

# 2. Check Core Scripts (Existence and End-of-File check)
check_file() {
    FILE=$1
    MARKER=$2
    if [ ! -f "$FILE" ]; then
        echo "[!!] MISSING: $FILE"
        ERRORS=$((ERRORS+1))
    elif ! grep -q "$MARKER" "$FILE"; then
        echo "[!!] TRUNCATED: $FILE (End marker '$MARKER' not found)"
        ERRORS=$((ERRORS+1))
    else
        echo "[OK] Script: $FILE"
    fi
}

check_file "$ROOT/agent_exec.sh" "exit \$EXIT_CODE"
check_file "$ROOT/heartbeat.sh" "done"
check_file "$ROOT/hud.py" "live.update"
check_file "$ROOT/agents/dev_01/stats.rs" "println!"
check_file "$ROOT/agents/dev_01/architect.py" "evolve()"

if [ $ERRORS -eq 0 ]; then
    echo ">> [SUCCESS] ALL COMPONENTS PRESENT AND COMPLETE."
else
    echo ">> [FAILURE] $ERRORS ERRORS DETECTED. RE-RUN MISSING BLOCKS."
fi
