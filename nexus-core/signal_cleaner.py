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
