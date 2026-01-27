import socket
import os
import time

SOCKET_PATH = os.path.expanduser("~/nexus_singularity/data/nexus.sock")

def run_hud():
    # Clear screen and set the stage
    print("\033[H\033[J") 
    print(">> [ SOVEREIGN HUD ] STATUS: NEURAL_LINK_ACTIVE")
    print(">> SOURCE: RAM_SOCKET (NEXUS.SOCK)")
    print("=" * 45)

    while True:
        try:
            with socket.socket(socket.AF_UNIX, socket.SOCK_STREAM) as s:
                s.settimeout(0.1)
                s.connect(SOCKET_PATH)
                data = s.recv(1024).decode()
                if data:
                    print(f"\r[ TELEMETRY ]: {data}          ", end="", flush=True)
        except (socket.error, socket.timeout):
            pass
        time.sleep(0.5)

if __name__ == "__main__":
    run_hud()
