import { json } from '@sveltejs/kit';
import fs from 'node:fs/promises';
import path from 'node:path';

export async function GET() {
    try {
        const currentDir = process.cwd();
        const entries = await fs.readdir(currentDir, { withFileTypes: true });
        
        const files = entries.map(entry => ({
            name: entry.name,
            is_dir: entry.isDirectory(),
            path: path.join(currentDir, entry.name)
        }));

        files.sort((a, b) => (a.is_dir === b.is_dir ? 0 : a.is_dir ? -1 : 1));

        return json(files);
    } catch (err) {
        return json({ error: err.message }, { status: 500 });
    }
}
