#!/bin/bash
while true; do
  echo "--- [HEALTH CHECK] $(date +%H:%M:%S) ---"
  lsof -Pi :8080 -sTCP:LISTEN -t >/dev/null && echo ">> 8080: ONLINE" || echo ">> 8080: OFFLINE"
  lsof -Pi :7340 -sTCP:LISTEN -t >/dev/null && echo ">> 7340: ONLINE" || echo ">> 7340: OFFLINE"
  sleep 60
done
