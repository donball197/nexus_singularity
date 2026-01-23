import os
import json

print('\n\033[1;36m>> COMMANDER INTEL BRIEFING\033[0m')
print('='*45)

agents_dir = os.path.expanduser('~/nexus_singularity/agents')
if not os.path.exists(agents_dir):
    print(">> [ERR] NO AGENT SANCTUARIES DETECTED")
else:
    for agent in sorted(os.listdir(agents_dir)):
        mem_file = os.path.join(agents_dir, agent, 'memory.json')
        if os.path.exists(mem_file):
            with open(mem_file, 'r') as f:
                try:
                    mem = json.load(f)
                    print(f'AGENT: {agent.upper()}')
                    for k, v in mem.items():
                        print(f'  - {k: <10}: {v}')
                except:
                    print(f'AGENT: {agent.upper()} [DATA CORRUPT]')
print('='*45)
