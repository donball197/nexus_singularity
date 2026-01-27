import socket
import os

# Create a Unix Socket (A memory pipe)
SOCKET_PATH = os.path.expanduser("~/nexus_singularity/data/nexus.sock")

if os.path.exists(SOCKET_PATH):
    os.remove(SOCKET_PATH)

server = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
server.bind(SOCKET_PATH)
server.listen(1)

print(">> [NEURAL BRIDGE] LISTENING IN RAM...")

while True:
    conn, _ = server.accept()
    data = conn.recv(1024)
    if data:
        # Received directly from Rust's memory
        print(f">> [RECEIVED VIA RAM]: {data.decode()}")
    conn.close()
