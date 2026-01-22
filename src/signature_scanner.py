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
