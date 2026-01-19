import sys, json, re
# Rules of Engagement
BLOCKLIST = ["rm -rf /", ":(){ :|:& };:", "mkfs", "dd if=/dev/zero"]

def audit(cmd):
    for danger in BLOCKLIST:
        if danger in cmd.lower():
            print(json.dumps({"status": "blocked", "reason": danger}))
            sys.exit(1)
    print(json.dumps({"status": "allowed"}))
    sys.exit(0)

if __name__ == "__main__":
    if len(sys.argv) > 1: audit(sys.argv[1])
