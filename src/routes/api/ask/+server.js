import { json } from '@sveltejs/kit';
import { exec } from 'node:child_process';
import { promisify } from 'node:util';
import { GEMINI_API_KEY } from '$env/static/private';
import { env } from '$env/dynamic/private'; 

const execAsync = promisify(exec);

export async function POST({ request }) {
    try {
        const { text } = await request.json();
        const apiKey = GEMINI_API_KEY;
        const modelId = env.MODEL_ID || 'gemma-3-27b-it';

        if (!apiKey) return json({ candidates: [{ content: { parts: [{ text: ">> [ERR] CONFIG ERROR: GEMINI_API_KEY missing." }] } }] });

        const systemPrompt = `You are COMMANDER GEMMA-3. System Version 0.5.1.
PROTOCOL:
- ACTION: SPAWN <name>
- ACTION: WRITE <agent> <file> <content>
- ACTION: READ <agent> <file>
- ACTION: EXEC <agent> <file>
RESTRICTION: Output ONLY the ACTION line.`;
        
        const response = await fetch(`https://generativelanguage.googleapis.com/v1beta/models/${modelId}:generateContent?key=${apiKey}`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ contents: [{ parts: [{ text: systemPrompt + "\n\nUSER: " + text }] }] })
        });

        const data = await response.json();
        if (data.error) return json({ candidates: [{ content: { parts: [{ text: ">> [ERR] API: " + data.error.message }] } }] });

        let answer = data.candidates?.[0]?.content?.parts?.[0]?.text || ">> [ERR] Silence";
        answer = answer.trim();

        // --- INTELLIGENT NORMALIZATION ---
        // If Gemma forgets "ACTION:", we add it back manually.
        if (answer.startsWith("SPAWN")) answer = "ACTION: " + answer;
        if (answer.startsWith("WRITE")) answer = "ACTION: " + answer;
        if (answer.startsWith("READ"))  answer = "ACTION: " + answer;
        if (answer.startsWith("EXEC"))  answer = "ACTION: " + answer;
        if (answer.startsWith("RUN"))   answer = "ACTION: EXEC " + answer.replace("RUN ", ""); // Map RUN to EXEC

        // --- 1. SPAWN ---
        if (answer.includes("ACTION: SPAWN")) {
            const name = answer.split("ACTION: SPAWN")[1].trim().split(/\s+/)[0].replace(/["']/g, "");
            try { await execAsync(`bash ~/nexus_singularity/spawn_sibling.sh ${name}`); answer += `\n>> [KERNEL] SPAWN: ${name}`; } 
            catch(e) { answer += `\n>> [ERR] ${e.message}`; }
        }

        // --- 2. WRITE (Robust) ---
        if (answer.includes("ACTION: WRITE")) {
            const raw = answer.split("ACTION: WRITE")[1].trim();
            const firstSpace = raw.indexOf(' ');
            if (firstSpace > 0) {
                const agent = raw.substring(0, firstSpace);
                const rest = raw.substring(firstSpace + 1).trim();
                const secondSpace = rest.indexOf(' ');
                
                if (secondSpace > 0) {
                    const filename = rest.substring(0, secondSpace);
                    let content = rest.substring(secondSpace + 1).trim();

                    // Cleaning
                    if (content.startsWith('"') && content.endsWith('"')) content = content.slice(1, -1);
                    content = content.replace(/\\"/g, '"')
                                     .replace(/```[a-z]*\n?/gi, "")
                                     .replace(/```/g, "")
                                     .split("\\n").join("\n");

                    const b64 = Buffer.from(content).toString('base64');
                    try { 
                        const { stdout } = await execAsync(`bash ~/nexus_singularity/agent_write.sh ${agent} ${filename} ${b64}`); 
                        answer += `\n\n${stdout}`; 
                    } catch(e) { answer += `\n>> [ERR] ${e.message}`; }
                }
            }
        }

        // --- 3. READ ---
        if (answer.includes("ACTION: READ")) {
            const parts = answer.split("ACTION: READ")[1].trim().split(/\s+/);
            try { const { stdout } = await execAsync(`bash ~/nexus_singularity/agent_read.sh ${parts[0]} ${parts[1]}`); answer += `\n\n${stdout}`; } 
            catch(e) { answer += `\n>> [ERR] 404: File not found`; }
        }

        // --- 4. EXEC ---
        if (answer.includes("ACTION: EXEC")) {
            const parts = answer.split("ACTION: EXEC")[1].trim().split(/\s+/);
            try { const { stdout, stderr } = await execAsync(`bash ~/nexus_singularity/agent_exec.sh ${parts[0]} ${parts[1]}`); answer += `\n\n${stdout || stderr}`; } 
            catch(e) { answer += `\n>> [ERR] ${e.message}`; }
        }

        data.candidates[0].content.parts[0].text = answer;
        return json(data);
    } catch (err) {
        return json({ error: err.message }, { status: 500 });
    }
}
