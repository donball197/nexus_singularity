import os, subprocess
from flask import Flask, request, jsonify
from flask_cors import CORS

app = Flask(__name__)
CORS(app)
LEDGER_PATH = os.path.expanduser("~/nexus_singularity/nexus_memory/LEDGER.md")

@app.route('/command', methods=['POST'])
def handle_command():
    data = request.json
    cmd_text = data.get("command", "").strip()
    if cmd_text.startswith("!"):
        shell_cmd = cmd_text[1:]
        try:
            result = subprocess.check_output(shell_cmd, shell=True, stderr=subprocess.STDOUT).decode()
            response_msg = result if result else "Success"
        except Exception as e:
            response_msg = f"Error: {str(e)}"
    else:
        response_msg = f"Logged: {cmd_text}"
        os.system("termux-vibrate -d 50")
    
    with open(LEDGER_PATH, "a") as f:
        f.write(f"- {cmd_text}\n")
    return jsonify({"status": "success", "output": response_msg})

if __name__ == '__main__':
    app.run(host='0.0.0.0', port=8081)
