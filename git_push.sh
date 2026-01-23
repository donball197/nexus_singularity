#!/bin/bash
# TARGET URL confirmed by Commander
REMOTE_URL="https://github.com/donball197/nexus_singularity.git"

cd ~/nexus_singularity

# 1. Connect to the remote sector
# We use 'set-url' in case 'origin' was already partially added
git remote add origin "$REMOTE_URL" 2>/dev/null || git remote set-url origin "$REMOTE_URL"

# 2. Standardize branch identity
git branch -M main

# 3. Secure Push
# We use -f (force) because the remote repo has an unrelated history
echo -e "\033[1;32m>> [KERNEL] INITIATING FORCE UPLINK TO REMOTE...\033[0m"
git push -f -u origin main

echo -e "\033[1;32m>> [SUCCESS] REPOSITORY SYNCHRONIZED.\033[0m"
