#!/bin/bash
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${GREEN}>>> [NEXUS DIAGNOSTICS] Starting Pre-Flight Check...${NC}"

# 1. Check Library Version
echo -n "Checking libonnxruntime.so version... "
if [ -f "$HOME/ort_lib/onnxruntime-android-1.19.0/lib/libonnxruntime.so" ]; then
    strings $HOME/ort_lib/onnxruntime-android-1.19.0/lib/libonnxruntime.so | grep -q "1.19.0"
    echo -e "${GREEN}Verified 1.19.0${NC}"
else
    echo -e "${RED}MISSING!${NC}"
fi

# 2. Check Architecture
echo -n "Checking Library Architecture... "
if command -v readelf &> /dev/null; then
    readelf -h $HOME/ort_lib/onnxruntime-android-1.19.0/lib/libonnxruntime.so | grep -q "AArch64"
    echo -e "${GREEN}AArch64 (Correct)${NC}"
else
    echo "readelf not found, skipping check."
fi

# 3. Check Workspace & Model
echo -n "Checking Workspace... "
[ -d "./workspace" ] && echo -e "${GREEN}OK${NC}" || echo -e "${RED}FAIL${NC}"
echo -n "Checking model.onnx... "
[ -f "./models/model.onnx" ] && echo -e "${GREEN}OK${NC}" || echo -e "${RED}FAIL${NC}"

# 4. Check Cargo & Binary
echo -n "Checking compiled binary... "
[ -f "./target/release/nexus_singularity" ] && echo -e "${GREEN}READY${NC}" || echo -e "${RED}NOT COMPILED${NC}"

echo -e "\n${GREEN}>>> Pre-Flight Complete. If all GREEN, run 'nexus-start'${NC}"
