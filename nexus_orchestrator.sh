#!/bin/bash
# NEXUS ORCHESTRATOR v1.0 (LENOVO NODE)

echo ">> [SYSTEM] IGNITING LENOVO NEURAL NODE..."

# 1. Clean Registry
echo '{"node_id":"LENOVO_DUET_01","status":"READY","agents":{}}' > ~/nexus_singularity/agent_registry.json

# 2. Launch Universal Agents
python3 ~/nexus_singularity/src/sentinel_agent.py &
python3 ~/nexus_singularity/src/forge_master.py &
python3 ~/nexus_singularity/src/gemma_supervisor.py &

# 3. Dual-Mode Communication
# Start the Ear (Listener)
python3 ~/nexus_singularity/src/ghost_trigger.py &
# Start the Muscle (Broadcaster)
./target/release/nexus_singularity &

echo ">> [SYSTEM] LENOVO IS NOW SYMMETRIC. MONITORING FOR MOTOROLA..."
