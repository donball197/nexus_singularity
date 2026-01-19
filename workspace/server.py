import http.server
import socketserver
import os

# Define the port (8000 is standard, Nexus is on 8080)
PORT = 8000
# Ensure we serve the workspace directory, not the backend root
DIRECTORY = "workspace"

class Handler(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=DIRECTORY, **kwargs)

print(f"🚀 Serving web files at http://localhost:{PORT}")

with socketserver.TCPServer(("", PORT), Handler) as httpd:
    httpd.serve_forever()
