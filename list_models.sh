#!/bin/bash
# Lazarus Hive v0.5.4 - Intelligence Scout (.env Aware)

# 1. Load the Hive Identity from .env
if [ -f .env ]; then
    export $(grep -v '^#' .env | xargs)
    echo ">> [INFO] Hive Identity loaded from .env"
fi

# 2. Safety Check
if [ -z "$GEMINI_API_KEY" ]; then
    echo ">> [ERROR] GEMINI_API_KEY not found in .env or system."
    exit 1
fi

echo ">> SCOUTING LIVE INTELLIGENCE OPTIONS..."
curl -s "https://generativelanguage.googleapis.com/v1beta/models?key=$GEMINI_API_KEY" \
    | jq -r '.models[] | "\(.name) | \(.displayName) | Context: \(.inputTokenLimit)"' \
    | column -t -s "|"

echo ">> SCOUT COMPLETE. BREADCRUMBS DROPPED."
