import { text } from '@sveltejs/kit';
import fs from 'node:fs/promises';

export async function GET({ url }) {
    try {
        const filePath = url.searchParams.get('path');
        if (!filePath) return text('No path provided', { status: 400 });
        
        const content = await fs.readFile(filePath, 'utf-8');
        return text(content);
    } catch (err) {
        return text(`Error reading file: ${err.message}`, { status: 500 });
    }
}
