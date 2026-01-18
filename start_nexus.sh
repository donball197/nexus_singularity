#!/bin/bash
echo "🔋 Status: $(termux-battery-status | grep percentage | awk '{print $2}' | sed 's/,//')%"
echo "🔥 Temp: $(termux-battery-status | grep temperature | awk '{print $2}' | sed 's/,//')°C"

# Check RAM safety before ignition
FREE_RAM=$(free -m | awk '/Mem:/ {print $4}')
if [ $FREE_RAM -lt 100 ]; then
    echo "❌ ERROR: Insufficient RAM ($FREE_RAM MB). Close other apps!"
    exit 1
fi

echo "🚀 Igniting Nexus Lite Core..."
# Run the binary pulled from Cloud Forge
./backend
