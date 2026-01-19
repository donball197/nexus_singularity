#!/data/data/com.termux/files/usr/bin/bash
echo "🔥 IGNITING UNIFIED SOVEREIGN KERNEL..."
termux-wake-lock
pkill -9 -f nexus_singularity || true
fuser -k 8080/tcp || true
sleep 1
while true; do
    ./target/release/nexus_singularity 2>&1 | tee -a server.log
    echo ">> CRASH_DETECTED. RESTARTING_IN_5S..."
    sleep 5
done
