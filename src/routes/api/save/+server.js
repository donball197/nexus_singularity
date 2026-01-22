import { json } from '@sveltejs/kit';
import fs from 'node:fs/promises';

export async function POST({ request }) {
    try {
        const { path, content } = await request.json();
        await fs.writeFile(path, content, 'utf-8');
        return json({ success: true });
    } catch (err) {
        return json({ error: err.message }, { status: 500 });
    }
}
