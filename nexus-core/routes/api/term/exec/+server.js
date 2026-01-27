import { text } from '@sveltejs/kit';
import { exec } from 'node:child_process';

export async function POST({ request }) {
    const { cmd } = await request.json();
    return new Promise((resolve) => {
        exec(cmd, { cwd: process.cwd() }, (error, stdout, stderr) => {
            if (error) {
                resolve(text(stderr || error.message));
            } else {
                resolve(text(stdout || "Done."));
            }
        });
    });
}
