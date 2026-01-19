import sys
import json
import subprocess

# STANDARD LIB ONLY - NO PIP NEEDED
def classify(prompt):
    # CRITICAL FIX: If this is a System Command, PASS it to Rust/Gemini.
    # Don't try to be smart if the user is forcing a tool.
    if prompt.strip().startswith("["):
        return {"action": "pass", "model": "cloud"}

    p = prompt.lower()
    
    # 1. Hardware Reflex
    if any(x in p for x in ["battery", "charge", "power"]):
        try:
            out = subprocess.check_output("termux-battery-status", shell=True).decode('utf-8')
            return {"action": "reply", "content": f"BATTERY: {out}"}
        except:
            return {"action": "reply", "content": "BATTERY: Sensor Offline"}

    # 2. Date/Time Reflex
    if "time" in p or "date" in p:
        from datetime import datetime
        return {"action": "reply", "content": str(datetime.now())}

    # 3. Default to Cloud
    return {"action": "pass", "model": "cloud"}

if __name__ == "__main__":
    if len(sys.argv) > 1:
        print(json.dumps(classify(sys.argv[1])))
    else:
        print(json.dumps({"error": "No input"}))
