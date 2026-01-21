#!/bin/bash
# Project Nexus Singularity - Model Scout v1.0
# Fetches live Google AI models using your Sovereign API Key

if [ -z "$GEMINI_API_KEY" ]; then
    echo "Error: GEMINI_API_KEY is not set. Source your .env_nexus first."
    exit 1
fi

echo "Fetching current Google models..."
curl -s "https://generativelanguage.googleapis.com/v1beta/models?key=${GEMINI_API_KEY}" \
    | jq -r '.models[] | "\(.name) - \(.description)"' | sed 's/models\///g'
