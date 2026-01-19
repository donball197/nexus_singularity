#!/bin/bash
# NEXUS MECHANIC TOOLCHAIN SETUP
echo ">> UPDATING DEBIAN REPOSITORIES..."
apt update && apt upgrade -y

echo ">> INSTALLING COMPILERS (C/C++)..."
apt install -y build-essential gcc g++ gdb make cmake

echo ">> INSTALLING PYTHON STACK..."
apt install -y python3 python3-pip python3-venv

echo ">> INSTALLING UTILITIES..."
apt install -y git curl wget zip unzip

echo ">> CLEANING UP..."
apt autoremove -y

echo ">> TOOLCHAIN READY: GCC $(gcc --version | head -n 1)"
echo ">> TOOLCHAIN READY: Python $(python3 --version)"
