import time
import sys

def translate_pulses():
    print(">> [NEURAL] TRANSLATOR ACTIVE: LISTENING FOR BITS...")
    try:
        while True:
            t0 = time.perf_counter()
            time.sleep(0.01)
            delta = time.perf_counter() - t0
            
            if delta > 0.015: # We found a pulse
                # If pulse is long (>0.2s), it's a 1. If short, it's a 0.
                bit = "1" if delta > 0.2 else "0"
                sys.stdout.write(bit)
                sys.stdout.flush()
    except KeyboardInterrupt:
        print("\n>> [NEURAL] TRANSLATOR STANDBY.")

if __name__ == "__main__":
    translate_pulses()
