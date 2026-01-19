import sys
import json
import os

# --- FIXED TEMPLATE (Single Braces) ---
AGENT_TEMPLATE = """
import sys
import json

def task(args):
    # Agent Logic Goes Here
    # Note: {name} is replaced by the factory. {args} stays as a variable.
    return { "status": "success", "output": f"Agent {name} executed with: {args}" }

if __name__ == "__main__":
    try:
        args = sys.argv[1:] if len(sys.argv) > 1 else []
        result = task(args)
        print(json.dumps(result))
    except Exception as e:
        print(json.dumps({ "status": "error", "message": str(e) }))
"""

def spawn_agent(agent_name, role_description):
    filename = f"workspace/agent_{agent_name.lower()}.py"
    
    # Inject the name into the template
    code = AGENT_TEMPLATE.replace("{name}", agent_name)
    
    with open(filename, "w") as f:
        f.write(code)
    
    return f"Spawned Agent: {agent_name} at {filename}"

if __name__ == "__main__":
    if len(sys.argv) < 3:
        print(json.dumps({"status": "error", "message": "Usage: factory.py <name> <role>"}))
        sys.exit(1)

    name = sys.argv[1]
    role = sys.argv[2]
    
    try:
        msg = spawn_agent(name, role)
        print(json.dumps({"status": "success", "message": msg}))
    except Exception as e:
        print(json.dumps({"status": "error", "message": str(e)}))
