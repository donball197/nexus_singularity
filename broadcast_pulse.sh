#!/bin/bash
# Project Nexus Singularity: Pulse Broadcaster (V0.3.8)

# Verified Duet IP from termux-wifi-connectioninfo
TARGET_IP="192.168.1.38" 
PORT=7340

echo "🚀 Starting Pulse Broadcast to $TARGET_IP:$PORT..."
while true; do
  MESSAGE="⚡ MESH_PULSE: FIELD_01_ACTIVE | $(date +%T)"
  echo -n "$MESSAGE" | nc -u -w1 $TARGET_IP $PORT
  echo "Sent: $MESSAGE"
  sleep 5
done
