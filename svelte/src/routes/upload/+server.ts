import { writeFile } from 'fs/promises';
import path from 'path';
import { RequestHandler } from '@sveltejs/kit';

export const POST: RequestHandler = async ({ request }) => {
    try {
        const formData = await request.formData();
        const file = formData.get('file'); // 'file' is the name of the input field

        if (!file || !(file instanceof Blob)) {
            return new Response(JSON.stringify({ success: false, message: 'No file provided' }), {
                status: 400,
                headers: { 'Content-Type': 'application/json' },
            });
        }

        const arrayBuffer = await file.arrayBuffer();
        const buffer = Buffer.from(arrayBuffer);

        // Define the upload directory and ensure it exists
        const uploadDir = path.resolve('static/uploads');
        const filePath = path.join(uploadDir, file.name);

        // Save the file
        await writeFile(filePath, buffer);

        return new Response(
            JSON.stringify({ success: true, message: 'File uploaded successfully', filePath }),
            { headers: { 'Content-Type': 'application/json' } }
        );
    } catch (err) {
        console.error('Error uploading file:', err);
        return new Response(
            JSON.stringify({ success: false, message: 'Internal Server Error' }),
            { status: 500, headers: { 'Content-Type': 'application/json' } }
        );
    }
};
