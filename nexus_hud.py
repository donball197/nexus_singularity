import time
from rich.console import Console
from rich.layout import Layout
from rich.panel import Panel
from rich.syntax import Syntax
from rich.table import Table
from rich.live import Live

console = Console()

def make_layout():
    layout = Layout()
    layout.split_column(
        Layout(name="header", size=3),
        Layout(name="body"),
        Layout(name="footer", size=3),
    )
    layout["body"].split_row(
        Layout(name="telemetry", ratio=1),
        Layout(name="logs", ratio=2),
    )
    return layout

def get_numbered_logs():
    # This reads your alerts.log and adds line numbers for the HUD
    try:
        with open("/home/donball197/nexus_singularity/agents/dev_01/alerts.log", "r") as f:
            content = f.read()
        return Syntax(content, "text", line_numbers=True, theme="monokai")
    except:
        return ">> WAITING FOR DATA..."

# --- HUD REFRESH LOOP ---
layout = make_layout()
layout["header"].update(Panel("NEXUS SINGULARITY : COMMAND HUD", style="bold cyan"))
layout["footer"].update(Panel("PRESS CTRL+C TO EXIT", style="dim"))

with Live(layout, refresh_per_second=1, screen=True):
    while True:
        layout["logs"].update(Panel(get_numbered_logs(), title="[ Tactical Triage ]"))
        time.sleep(1)
