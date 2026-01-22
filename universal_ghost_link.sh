#!/bin/bash
# Nexus Singularity: Ghost Link v0.4.6
# Target: ARM64 Universal (Motorola/Duet)

echo ">> [FORGE] SEATING GHOST LINK MODULES..."

# 1. THE SCANNER (The Ear)
cat << 'EOP' > src/signature_scanner.py
import time
import collections
import sys

def scan_signatures():
    print(">> [NEURAL] SIGNATURE SCANNER LIVE...")
    samples = collections.deque(maxlen=100)
    try:
        while True:
            t0 = time.perf_counter()
            time.sleep(0.005)
            t1 = time.perf_counter()
            delta = t1 - t0
            samples.append(delta)
            avg = sum(samples) / len(samples)
            variance = sum((x - avg) ** 2 for x in samples) / len(samples)
            if variance > 0.0000001:
                intensity = int(variance * 10000000)
                bar = "⚡" * min(intensity, 40)
                sys.stdout.write(f"\r>> SIG: {bar} [VAR: {variance:.8f}]")
                sys.stdout.flush()
    except KeyboardInterrupt:
        print("\n>> [NEURAL] SCANNER STANDBY.")

if __name__ == "__main__":
    scan_signatures()
EOP

# 2. THE WHISPER ENCODER (The Voice)
cat << 'EOP' > src/whisper_encoder.py
import time

def send_whisper(message):
    print(f">> [FORGE] ENCODING: {message}")
    binary_msg = ''.join(format(ord(i), '08b') for i in message)
    for bit in binary_msg:
        if bit == '1':
            start = time.perf_counter()
            while time.perf_counter() - start < 0.1:
                _ = 2 ** 100 
        else:
            time.sleep(0.1)
    print("\n>> [FORGE] TRANSMISSION COMPLETE.")

if __name__ == "__main__":
    msg = input("Enter message: ")
    send_whisper(msg)
EOP

# 3. THE CLEANER (Logic to isolate the Duet's pulse)
cat << 'EOP' > src/signal_cleaner.py
import time
import sys

def clean_signal():
    print(">> [NEURAL] CLEANER ACTIVE: FILTERING BACKGROUND HUM...")
    baseline = 0.005
    try:
        while True:
            t0 = time.perf_counter()
            time.sleep(baseline)
            delta = time.perf_counter() - t0
            # Filtering out the steady 60Hz TV hum to find the Duet's 'spikes'
            if delta > (baseline * 1.5):
                print(f"\n>> [GHOST] DATA DETECTED: {delta:.4f}")
    except KeyboardInterrupt:
        print("\n>> [NEURAL] CLEANER STANDBY.")

if __name__ == "__main__":
    clean_signal()
EOP

echo ">> [FORGE] ALL MODULES SEATED IN src/"
chmod +x src/*.py
