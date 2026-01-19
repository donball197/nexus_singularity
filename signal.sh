#!/data/data/com.termux/files/usr/bin/bash
# Project Nexus: Numeric Morse Transceiver
# Target: Node #1 (Localhost)

echo "📡 Signal Transceiver Active..."
echo "Press [1] for Pulse, [2] for Dash, [q] to Exit."
echo "------------------------------------------"

while :
do
    # Read a single character without needing 'Enter'
    read -n 1 -s key
    
    case $key in
        1)
            echo ">> Pulse [1] sent."
            curl -s -X POST http://127.0.0.1:8080/signal -d "{\"type\": \"numeric\", \"value\": 1}" > /dev/null
            ;;
        2)
            echo ">> Dash  [2] sent."
            curl -s -X POST http://127.0.0.1:8080/signal -d "{\"type\": \"numeric\", \"value\": 2}" > /dev/null
            ;;
        q)
            echo -e "\nTransceiver Offline."
            exit
            ;;
    esac
done
