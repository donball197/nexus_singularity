#!/bin/bash
cd ~/nexus_singularity

# 1. Initialize if needed
if [ ! -d ".git" ]; then
    git init -b main
    echo ">> [KERNEL] GIT REPOSITORY INITIALIZED."
fi

# 2. Status Check
echo ">> [SYSTEM] SCANNING FOR SENSITIVE LEAKS..."
if grep -q "AIza" README.md src/routes/api/ask/+server.js; then
    echo ">> [ERR] SECURITY BREACH: API KEY DETECTED IN SOURCE. ABORTING."
    exit 1
fi

# 3. Commit
git add .
git commit -m "COMMANDER GEMMA-3: Infrastructure finalized. Security perimeter active."
echo ">> [SUCCESS] LOCAL COMMIT COMPLETE."
