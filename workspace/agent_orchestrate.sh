#!/data/data/com.termux/files/usr/bin/bash

# 1. Handshake with the Termux Environment
termux-vibrate -d 200
echo ">> NEXUS ORACLE: GENERATING 24-HOUR STRATEGY MANIFEST..."
echo "========================================================"

# 2. Extract the "Gold" from the Librarian's Vault
LEDGER="/data/data/com.termux/files/home/nexus_master/DOCS/LEDGER.md"

if [ ! -f "$LEDGER" ]; then
    echo "!! ERROR: Ledger not found. Ensure Librarian is active."
    exit 1
fi

# 3. Filter for High-Value Signals
echo "CRITICAL SIGNALS DETECTED:"
grep "GOLD SIGNAL" "$LEDGER" | tail -n 10 | sed 's/## GOLD SIGNAL: //'

echo "--------------------------------------------------------"

# 4. Oracle Reasoning Output
echo "ORACLE STRATEGY DIRECTIVE:"
echo "Current Focus: Sustainable Hardware Acquisition (Hardware Waste)."
echo "Status: Sovereignty Maintained on Budget Metal."
echo "========================================================"
termux-tts-speak "Manifest generated. 8th layer strategy is active."
