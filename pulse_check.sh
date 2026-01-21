#!/bin/bash
FREE_RAM=$(free -m | awk '/^Mem:/{print $4}')
cat << JSON > ~/nexus_singularity/static/neural_goal.json
{
  "node_id": "LENOVO_COMMAND",
  "current_objective": "Revenue Monitoring",
  "available_ram": "${FREE_RAM}MB",
  "neural_state": "Active",
  "last_sync": "$(date +'%H:%M:%S')"
}
JSON
echo ">> PULSE: Neural Goal updated."
