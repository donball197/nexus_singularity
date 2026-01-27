import time
import json
import os
from rich.console import Console
from rich.layout import Layout
from rich.panel import Panel
from rich.syntax import Syntax
from rich.live import Live

console = Console()
INTERCOM_PATH = os.path.expanduser("~/nexus_singularity/agents/dev_01/intercom.json")

def get_intercom_display():
    if not os.path.exists(INTERCOM_PATH):
        return ">> WAITING FOR INTERCOM..."
    
    with open(INTERCOM_PATH, "r") as f:
        # We read the raw text to preserve line numbers in the HUD
        content = f.read()
    
    # This renders the JSON with syntax highlighting and LINE NUMBERS
    return Syntax(content, "json", line_numbers=True, theme="monokai", word_wrap=True)

# --- HUD Setup ---
layout = Layout()
layout.split_column(
    Layout(name="header", size=3),
    Layout(name="body"),
)
layout["header"].update(Panel("NEXUS COMMAND : INTERCOM MONITOR", style="bold green"))

with Live(layout, refresh_per_second=2, screen=True):
    while True:
        layout["body"].update(Panel(get_intercom_display(), title="[ Shared Memory Bus ]"))
        time.sleep(0.5)
