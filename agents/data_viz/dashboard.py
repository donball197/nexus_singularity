import os

def draw_chart(data, label, width=40):
    try:
        val = float(data)
        # Normalize: CPU load 0.0 to 1.0 (or more)
        bar_len = int(min(val * width, width))
        bar = '█' * bar_len + '-' * (width - bar_len)
        return f'{label: <10}: [{bar}] {val}'
    except:
        return f'{label: <10}: [DATA ERROR]'

try:
    path = os.path.expanduser('~/nexus_singularity/agents/dev_01/stats.csv')
    if not os.path.exists(path):
        print(">> [ERR] 404: TELEMETRY OFFLINE (stats.csv not found)")
    else:
        with open(path, 'r') as f:
            lines = f.readlines()
            if not lines:
                print(">> [ERR] DATA STREAM EMPTY")
            else:
                last = lines[-1].strip().split(',')
                if len(last) >= 3:
                    ts, cpu, mem = last[0], last[1], last[2]
                    print('\n\033[1;32m>> TACTICAL MISSION DASHBOARD\033[0m')
                    print('='*45)
                    print(f'TIMESTAMP  : {ts}')
                    print(draw_chart(cpu, 'CPU LOAD'))
                    print(f'AVAILABLE  : {mem} KB')
                    print('='*45)
except Exception as e:
    print(f'>> [SYSTEM ERR] {e}')
