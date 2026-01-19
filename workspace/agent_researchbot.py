
import sys
import json

def task(args):
    # Agent Logic Goes Here
    # Note: ResearchBot is replaced by the factory. {args} stays as a variable.
    return { "status": "success", "output": f"Agent ResearchBot executed with: {args}" }

if __name__ == "__main__":
    try:
        args = sys.argv[1:] if len(sys.argv) > 1 else []
        result = task(args)
        print(json.dumps(result))
    except Exception as e:
        print(json.dumps({ "status": "error", "message": str(e) }))
