import os, json, csv, sys
HOME = os.path.expanduser("~")
def check():
    conf_p = f"{HOME}/nexus_singularity/agents/dev_01/config.json"
    stat_p = f"{HOME}/nexus_singularity/agents/dev_01/stats.csv"
    with open(conf_p, 'r') as f: config = json.load(f)
    if not os.path.exists(stat_p): sys.exit(0)
    with open(stat_p, 'r') as f:
        last = f.readlines()[-1].split(',')
        load = float(last[1])
        if load > config['threshold']: sys.exit(2)
if __name__ == "__main__": check()
