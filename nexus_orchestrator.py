import requests
import os

# The bridge address pointing to the Termux Rust service
NEXUS_URL = "http://127.0.0.1:3000/ask"
# The shared directory we mapped with --bind
WORKSPACE_DIR = "/mnt/nexus_singularity/workspace"

def scan_and_analyze():
    if not os.path.exists(WORKSPACE_DIR):
        print(f"❌ Error: {WORKSPACE_DIR} not found.")
        return

    files = os.listdir(WORKSPACE_DIR)
    print(f"📂 Found {len(files)} files in workspace. Sending to Neural Core...")

    for file in files:
        payload = {"prompt": f"Analyze file: {file}"}
        try:
            response = requests.post(NEXUS_URL, json=payload)
            if response.status_code == 200:
                print(f"✅ {file}: {response.json().get('output')}")
            else:
                print(f"⚠️ {file}: Failed with status {response.status_code}")
        except Exception as e:
            print(f"❌ Bridge Connection Failed: {e}")

if __name__ == "__main__":
    scan_and_analyze()
