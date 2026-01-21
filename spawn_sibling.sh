#!/bin/bash
# Project Nexus Singularity - Sibling Dispatcher v0.4.2
# SAFETY: OOM-Guarded. API for High-IQ, Local for Nano.

source ~/.env_nexus

TASK_TYPE=$1

case $TASK_TYPE in
    "intelligence")
        # API ONLY: High-IQ Architect logic (Zero Crash Risk)
        export ACTIVE_SIBLING=$COMMANDER_MODEL
        export USE_API=true
        echo "Commander: Delegating to Specialist Sibling via API..."
        ;;
    "local")
        # LOCAL ONLY: Ultra-light 1B-2B models (No OOM)
        export ACTIVE_SIBLING=$TARGET_SIBLING
        export USE_API=false
        echo "Commander: Spawning Nano Sibling locally..."
        ;;
    *)
        # Default: Conservation Mode (Local Nano)
        export ACTIVE_SIBLING=$TARGET_SIBLING
        export USE_API=false
        echo "Commander: Defaulting to Local Nano Sibling."
        ;;
esac
