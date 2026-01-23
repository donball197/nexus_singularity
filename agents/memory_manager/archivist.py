import json
import os
import sys

def update_memory(agent_name, key, value):
    agent_dir = os.path.expanduser(f'~/nexus_singularity/agents/{agent_name}')
    if not os.path.exists(agent_dir):
        os.makedirs(agent_dir)
    
    mem_path = os.path.join(agent_dir, 'memory.json')
    
    # Load existing or create new
    if os.path.exists(mem_path):
        try:
            with open(mem_path, 'r') as f:
                memory = json.load(f)
        except:
            memory = {}
    else:
        memory = {}

    # Update state
    memory[key] = value
    
    with open(mem_path, 'w') as f:
        json.dump(memory, f, indent=4)
    print(f'>> [MEMORY] Updated {agent_name} state: {key}')

if __name__ == '__main__':
    if len(sys.argv) < 4:
        print('Usage: archivist.py <agent> <key> <value>')
    else:
        update_memory(sys.argv[1], sys.argv[2], sys.argv[3])
