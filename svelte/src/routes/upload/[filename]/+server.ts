// routes/uploads/[filename]/+server.ts
import { createReadStream } from 'fs';
import { stat } from 'fs/promises';
import path from 'path';
import { env } from '$env/dynamic/private';
import type { RequestHandler } from './$types';

export const GET: RequestHandler = async ({ params }) => {
    const filePath = path.join(env.UPLOAD_DIR || '/app/uploads', params.filename);

    try {
        const stats = await stat(filePath);
        if (!stats.isFile()) {
            return new Response('Not found', { status: 404 });
        }

        // Convert Node.js stream to Web stream
        const stream = createReadStream(filePath);
        const webStream = new ReadableStream({
            start(controller) {
                stream.on('data', (chunk) => controller.enqueue(chunk));
                stream.on('end', () => controller.close());
                stream.on('error', (err) => controller.error(err));
            }
        });

        return new Response(webStream);
    } catch {
        return new Response('Not found', { status: 404 });
    }
};