#!/bin/bash
echo "--- [SHIELD] INITIALIZING SAFE BOOT ---"
if [[ -d "/data/data/com.termux" ]]; then
    export NODE_TYPE="MOTOROLA_EDGE"
else
    export NODE_TYPE="LENOVO_COMMAND"
fi
echo "Node Active: $NODE_TYPE"
export XDG_SESSION_TYPE=wayland
export WAYLAND_DISPLAY=wayland-0
export DISPLAY=:0
echo ">> [OPTICS] Wayland/X11 Bridge: LOCKED"
