#!/bin/bash
# Minimalist model check
curl -s "https://generativelanguage.googleapis.com/v1beta/models?key=$GEMINI_API_KEY" \
| grep '"name":' \
| sed 's/.*models\///g' | sed 's/",//g'
