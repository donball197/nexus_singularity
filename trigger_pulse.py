import socket
import json

# Target: The LDuet Frequency Ear
TARGET_IP = "127.0.0.1"
PORT = 7340

# Neural Data Packet
data = {
    "sender": "MICRO_AGENT_01",
    "command": "PULSE_CHECK",
    "architect": "donball197",
    "payload": "Gemma 3 Sync Verified"
}

# Sending the UDP pulse
sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
sock.sendto(json.dumps(data).encode(), (TARGET_IP, PORT))

print(f">> [PULSE] Data packet injected to Port {PORT}")
