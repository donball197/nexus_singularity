```python
import os
import json
import openai

class NexusAgent:
    def __init__(self, model="gpt-4o-mini"):
        self.client = openai.OpenAI(api_key=os.getenv("OPENAI_API_KEY"))
        self.model = model
        self.memory = [{"role": "system", "content": "You are NEXUS. Concise. Mobile-First."}]

    def add_tool(self, func):
        """Placeholder for tool registration logic."""
        pass

    def run(self, prompt):
        self.memory.append({"role": "user", "content": prompt})
        
        response = self.client.chat.completions.create(
            model=self.model,
            messages=self.memory
        )
        
        reply = response.choices[0].message.content
        self.memory.append({"role": "assistant", "content": reply})
        return reply

if __name__ == "__main__":
    # Usage: export OPENAI_API_KEY='your-key'
    nexus = NexusAgent()
    while True:
        user_input = input("NEXUS > ")
        if user_input.lower() in ["exit", "quit"]: break
        print(f"\n{nexus.run(user_input)}\n")
```