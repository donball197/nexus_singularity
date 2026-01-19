import os
import shutil
import subprocess

# Configuration
SOURCE_DIR = "/data/data/com.termux/files/home/nexus_master"
DIST_DIR = "/data/data/com.termux/files/home/nexus_dist_package"

def build_package():
    print(">> [DIST AGENT]: Initializing Sanitization...")
    if os.path.exists(DIST_DIR):
        shutil.rmtree(DIST_DIR)
    os.makedirs(DIST_DIR)

    # 1. Copy the pre-compiled binary
    binary_path = f"{SOURCE_DIR}/target/aarch64-linux-android/release/nexus_bridge"
    if os.path.exists(binary_path):
        shutil.copy(binary_path, DIST_DIR)
        print(">> Binary injected.")
    else:
        print("!! Error: Binary not found. Build the project first.")
        return

    # 2. Copy core logic and UI
    shutil.copy(f"{SOURCE_DIR}/index.html", DIST_DIR)
    shutil.copy(f"{SOURCE_DIR}/watcher_agent.py", DIST_DIR)
    shutil.copy(f"{SOURCE_DIR}/README.md", DIST_DIR)
    
    # 3. Create fresh empty directories
    os.makedirs(f"{DIST_DIR}/inbox")
    
    print(f">> [DIST AGENT]: Package complete at {DIST_DIR}")
    subprocess.run(["termux-tts-speak", "Client package is ready for delivery."])

if __name__ == "__main__":
    build_package()
