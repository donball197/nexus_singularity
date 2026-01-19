import os
import json

def fetch_deepseek_intel():
    print(">> INITIATING DEEPSEEK MARKET SCAN (VIA DDGR)...")
    query = "DeepSeek LLM 2026 performance benchmarks"
    cmd = f"ddgr --json --np --num 5 '{query}'"
    
    try:
        raw_data = os.popen(cmd).read()
        if not raw_data.strip():
            return ">> SYSTEM: Search returned no data. Check internet connection."
        
        results = json.loads(raw_data)
        report = "### 📡 NEXUS INTELLIGENCE REPORT: DEEPSEEK\n\n"
        for i, entry in enumerate(results, 1):
            report += f"{i}. **{entry['title']}**\n"
            report += f"   - Summary: {entry['abstract']}\n"
            report += f"   - URL: {entry['url']}\n\n"
        return report
    except Exception as e:
        return f">> CRITICAL ERROR: {str(e)}"

if __name__ == "__main__":
    print(fetch_deepseek_intel())
