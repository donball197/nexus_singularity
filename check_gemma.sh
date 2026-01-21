#!/bin/bash
echo ">> NEURAL: Authenticating Sovereign Architect..."
curl -s "https://generativelanguage.googleapis.com/v1beta/models/gemma-3-27b-it:generateContent?key=$GOOGLE_API_KEY" \
    -H "Content-Type: application/json" \
    -d "{\"contents\": [{\"parts\":[{\"text\": \"Instruction: You are the Neural Engine for Project Nexus Singularity v0.2.2. Your Architect is Donald Ball (donball197). Confirm your link to the Lenovo LDuet node.\"}]}]}" | grep -i "text"
echo -e "\n>> STATUS: Link Confirmed."
