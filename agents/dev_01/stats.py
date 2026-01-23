import psutil
import datetime
import os

# Target the local agent directory
path = os.path.expanduser('~/nexus_singularity/agents/dev_01/stats.csv')
cpu = psutil.cpu_percent(interval=1) / 100.0
mem = psutil.virtual_memory().available // 1024
ts = datetime.datetime.now().strftime('%Y-%m-%d %H:%M:%S')

# Append telemetry data
with open(path, 'a') as f:
    f.write(f"{ts},{cpu},{mem}\n")

print(f">> [SENSOR] DATA LOGGED: CPU {cpu} | MEM {mem}KB")
