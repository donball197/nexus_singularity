import time
import os

def draw_hud():
    os.system('clear')
    print("\x1b[1;35m" + "="*40)
    print("   PROJECT NEXUS SINGULARITY v0.2.2")
    print("="*40 + "\x1b[0m")
    print(f"NODE: LENOVO_DUET (MUSCLE)")
    print(f"ROLE: FIELD COMMAND HUD")
    print(f"INTENT: REPLICATE")
    print("-" * 40)
    print("\x1b[1;32m[SYSTEM STATUS]\x1b[0m FORGING DNA...")
    print("\x1b[1;36m[LAZARUS PULSE]\x1b[0m [###-------] 32%")
    print("-" * 40)

while True:
    draw_hud()
    time.sleep(2)
