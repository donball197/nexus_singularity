import os
import sys

def list_dir(path="."):
    """List files in current directory."""
    try:
        files = os.listdir(path)
        for f in files:
            size = os.path.getsize(f)
            print(f"{f:20} | {size} bytes")
    except Exception as e:
        print(f"Error: {e}")

if __name__ == "__main__":
    p = sys.argv[1] if len(sys.argv) > 1 else "."
    list_dir(p)