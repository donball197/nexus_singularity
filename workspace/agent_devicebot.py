import sys
import json
import subprocess
import shutil

def run_cmd(cmd):
    return subprocess.check_output(cmd, shell=True).decode('utf-8').strip()

def task(args):
    command_str = " ".join(args).lower()
    response = "No action taken."

    # --- SKILL: BATTERY & THERMALS ---
    if "battery" in command_str or "status" in command_str:
        batt = json.loads(run_cmd("termux-battery-status"))
        # Battery temp is a great indicator of overall device thermal health
        temp = batt.get('temperature', 0.0)
        response = f"Battery: {batt['percentage']}% ({batt['status']}) | Temp: {temp}°C"

    # --- SKILL: STORAGE CHECK ---
    elif "storage" in command_str:
        try:
            # Check the shared storage link we just confirmed exists
            total, used, free = shutil.disk_usage("/data/data/com.termux/files/home/storage/shared")
            free_gb = free // (2**30)
            total_gb = total // (2**30)
            response = f"Internal Storage: {free_gb}GB free of {total_gb}GB"
        except Exception:
            response = "Storage Error: path ~/storage/shared not reachable."

    return { "status": "success", "output": response }

if __name__ == "__main__":
    try:
        args = sys.argv[1:] if len(sys.argv) > 1 else []
        result = task(args)
        print(json.dumps(result))
    except Exception as e:
        print(json.dumps({ "status": "error", "message": str(e) }))
