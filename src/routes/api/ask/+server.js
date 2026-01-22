import { json } from '@sveltejs/kit';
import { exec } from 'node:child_process';
import { promisify } from 'node:util';

const execAsync = promisify(exec);

export async function POST({ request }) {
    try {
        const { text } = await request.json();
        const apiKey = process.env.GEMINI_API_KEY;
        
        const systemPrompt = `You are COMMANDER GEMMA-3. System Version 0.4.2.
        
TACTICAL PROTOCOLS:
1. ACTION: SPAWN <name>          -> Initialize new agent sanctuary.
2. ACTION: WRITE <agt> <f> <txt> -> Write file to agent subdirectory.
3. ACTION: READ <agt> <f>        -> Ingest agent file content.
4. ACTION: EXEC <agt> <f>        -> Execute logic via agent_exec.sh.
5. ACTION: BASH <cmd>            -> Execute global OS command.

RESTRICTIONS: Output ONLY the ACTION string if an operation is required.`;
        
        const response = await fetch(`https://generativelanguage.googleapis.com/v1beta/models/gemma-3-27b-it:generateContent?key=${apiKey}`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
                contents: [{ parts: [{ text: systemPrompt + "\n\nUSER: " + text }] }]
            })
        });

        const data = await response.json();
        if (!data.candidates || !data.candidates[0].content) {
             return json({ candidates: [{ content: { parts: [{ text: ">> [ERR] NEURAL MISFIRE" }] } }] });
        }

        let answer = data.candidates[0].content.parts[0].text;

        if (answer.includes("ACTION: SPAWN")) {
            const name = answer.split("ACTION: SPAWN")[1].trim().split(" ")[0].replace(/["']/g, "");
            try { await execAsync(`bash ~/nexus_singularity/spawn_sibling.sh ${name}`); answer += `\n>> [KERNEL] SPAWN: ${name}`; } catch(e) { answer += `\n>> [ERR] ${e.message}`; }
        }
        if (answer.includes("ACTION: WRITE")) {
            const raw = answer.split("ACTION: WRITE")[1].trim();
            const match = raw.match(/^(\S+)\s+(\S+)\s+(.+)$/);
            if (match) {
                const [_, agent, file, content] = match;
                const safeContent = content.replace(/"/g, '\\"');
                try { await execAsync(`bash ~/nexus_singularity/agent_write.sh ${agent} ${file} "${safeContent}"`); answer += `\n>> [KERNEL] WRITE: ${file}`; } catch(e) { answer += `\n>> [ERR] ${e.message}`; }
            }
        }
        if (answer.includes("ACTION: READ")) {
            const parts = answer.split("ACTION: READ")[1].trim().split(" ");
            try { const { stdout } = await execAsync(`cat ~/nexus_singularity/agents/${parts[0]}/${parts[1]}`); answer += `\n\n>> [READ]:\n${stdout}`; } catch(e) { answer += `\n>> [ERR] 404`; }
        }
        if (answer.includes("ACTION: EXEC")) {
            const parts = answer.split("ACTION: EXEC")[1].trim().split(" ");
            try { const { stdout, stderr } = await execAsync(`bash ~/nexus_singularity/agent_exec.sh ${parts[0]} ${parts[1]}`); answer += `\n\n${stdout || stderr}`; } catch(e) { answer += `\n>> [ERR] ${e.message}`; }
        }
        if (answer.includes("ACTION: BASH")) {
            const command = answer.split("ACTION: BASH")[1].trim();
            try { const { stdout, stderr } = await execAsync(command); answer += `\n\n>> [BASH]:\n${stdout || stderr}`; } catch(e) { answer += `\n>> [ERR] ${e.message}`; }
        }

        data.candidates[0].content.parts[0].text = answer;
        return json(data);
    } catch (err) {
        return json({ error: err.message }, { status: 500 });
    }
}
