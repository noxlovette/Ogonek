// routes/api/upload/+server.ts
import { writeFile } from 'fs/promises';
import path from 'path';
import { env } from '$env/dynamic/private';
import { randomUUID } from 'crypto';
import type { RequestHandler } from './$types';

export const POST: RequestHandler = async ({ request }) => {
    try {
        const formData = await request.formData();
        const file = formData.get('file') as File;

        if (!file) {
            return new Response('No file provided', { status: 400 });
        }

        // Generate unique filename
        const ext = path.extname(file.name);
        const filename = `${randomUUID()}${ext}`;
        const uploadDir = env.UPLOAD_DIR || '/app/uploads';
        const filePath = path.join(uploadDir, filename);

        // Convert File to ArrayBuffer and write to disk
        const arrayBuffer = await file.arrayBuffer();
        const buffer = Buffer.from(arrayBuffer);
        await writeFile(filePath, buffer);

        return new Response(JSON.stringify({ filename }), {
            headers: { 'Content-Type': 'application/json' }
        });
    } catch (error) {
        console.error('Upload error:', error);
        return new Response('Upload failed', { status: 500 });
    }
};