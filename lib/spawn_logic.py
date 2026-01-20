import os
from google import genai

def list_nexus_brains():
    api_key = os.environ.get("GEMINI_API_KEY")
    if not api_key:
        print("Error: GEMINI_API_KEY not set. Run 'export GEMINI_API_KEY=...'")
        return

    client = genai.Client(api_key=api_key)
    print("--- Active Gemini 3 / 2.5 Brain Discovery ---")
    for m in client.models.list():
        if 'generateContent' in m.supported_actions:
            print(f"ACTIVE MODEL: {m.name}")

if __name__ == "__main__":
    list_nexus_brains()
