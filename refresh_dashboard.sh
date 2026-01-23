#!/bin/bash
# Clear screen for a clean tactical view
clear
# Execute Hardware Telemetry Chart
python3 ~/nexus_singularity/agents/data_viz/dashboard.py
# Execute Intelligence Briefing (Memory)
python3 ~/nexus_singularity/agents/memory_manager/briefing.py
