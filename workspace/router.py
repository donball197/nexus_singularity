import sys
import os
import json
import glob
import subprocess

def list_agents():
    agents = {}
    # Use absolute path to be safe
    files = glob.glob("/data/data/com.termux/files/home/nexus-duet-native/nexus-backend/workspace/agent_*.py")
    
    # Fallback to relative if absolute fails
    if not files:
        files = glob.glob("workspace/agent_*.py")

    for f in files:
        name = os.path.basename(f).replace("agent_", "").replace(".py", "")
        agents[name] = f
    return agents

def pick_agent(prompt, agent_list):
    prompt_lower = prompt.lower()
    for name, path in agent_list.items():
        if name in prompt_lower:
            return name, path
    return None, None

if __name__ == "__main__":
    # Force stdout to flush immediately
    sys.stdout.reconfigure(line_buffering=True)

    if len(sys.argv) < 2:
        print(json.dumps({"status": "error", "message": "No prompt provided."}))
        sys.exit(1)

    user_prompt = sys.argv[1]
    agents = list_agents()
    
    agent_name, agent_path = pick_agent(user_prompt, agents)
    
    if agent_path:
        print(f">> ROUTER: Found Specialist [{agent_name}]")
        print(f">> ROUTER: Dispatching...")
        
        try:
            # Run the agent
            cmd = ["python3", agent_path, user_prompt]
            result = subprocess.check_output(cmd, stderr=subprocess.STDOUT)
            print(result.decode('utf-8'))
        except subprocess.CalledProcessError as e:
            print(f"AGENT_CRASH: {e.output.decode('utf-8')}")
        except Exception as e:
            print(f"ROUTER_ERROR: {str(e)}")
            
    else:
        print(json.dumps({
            "status": "warning", 
            "message": "No agent matched.", 
            "agents_found": list(agents.keys())
        }))
