#!/bin/bash
echo ">> NEXUS WATCHDOG INITIATED"
while true; do
  if ! pgrep -x "nexus_lite" > /dev/null; then
    echo "!! CRASH DETECTED - RELAUNCHING SENTINEL"
    pkill -9 -f proot
    cd ~/nexus-duet-native/nexus-backend
    # Optional: Update the vault with the crash event
    echo "{\"milestones\": [\"Watchdog Relaunch $(date)\"]}" >> workspace/crash_log.json
    cargo run --release --bin nexus_lite
  fi
  sleep 5
done
