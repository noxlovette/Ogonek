import { fail, type Actions, error } from "@sveltejs/kit";
import { Buffer } from 'node:buffer';
import { nanoid } from 'nanoid';
import { createHash } from 'node:crypto';
import { writeFile } from 'node:fs/promises';
import path from 'node:path';
import { env } from '$env/dynamic/private';

const MAX_FILE_SIZE = 5 * 1024 * 1024; // 5MB in bytes
const ALLOWED_MIMES = new Set([
    'image/jpeg',
    'image/png',
    'application/pdf',
    // Add more as needed
]);

// Utility to encode filename - we'll use base64url for storage efficiency
const encodeFileName = (fileName: string): string => {
    return Buffer.from(fileName).toString('base64url');
};

export const actions = {
    default: async ({ request }) => {
        try {
            const formData = await request.formData();
            const file = formData.get('file');

            // Yo, let's validate this thing properly! 
            if (!file || !(file instanceof Blob)) {
                return fail(400, {
                    success: false,
                    message: 'No file provided or invalid file type'
                });
            }

            // Size check
            if (file.size > MAX_FILE_SIZE) {
                return fail(400, {
                    success: false,
                    message: 'File too large, max 5MB allowed'
                });
            }

            // MIME type validation
            if (!ALLOWED_MIMES.has(file.type)) {
                return fail(400, {
                    success: false,
                    message: 'Unsupported file type'
                });
            }
            const fileId = nanoid();
            const salt = crypto.getRandomValues(new Uint8Array(16));
            const arrayBuffer = await file.arrayBuffer();
            const buffer = Buffer.from(arrayBuffer);

            const hash = createHash('sha256')
                .update(Buffer.concat([buffer, Buffer.from(salt)]))
                .digest('hex');

            const encodedName = encodeFileName(file.name);
            const filePath = path.join(env.UPLOAD_DIR, encodedName);

            await writeFile(filePath, buffer.toString('base64'));

            return {
                success: true,
                message: 'File uploaded successfully',
                fileId,
                originalName: file.name
            };

        } catch (err) {
            console.error('Error uploading file:', err);
            return error(500);
        }
    }
} satisfies Actions;