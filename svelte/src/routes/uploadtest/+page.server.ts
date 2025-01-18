import { fail, type Actions, error } from "@sveltejs/kit";
import path from 'path';
import { writeFile } from 'fs/promises';

export const actions = {
    default: async ({ request }) => {
        try {
            const formData = await request.formData();
            const file = formData.get('file');

            if (!file || !(file instanceof Blob)) {
                return fail(400, { message: 'No file provided' });
            }

            const arrayBuffer = await file.arrayBuffer();
            const buffer = Buffer.from(arrayBuffer);

            const uploadDir = path.resolve('static/uploads');
            const filePath = path.join(uploadDir, file.name);

            await writeFile(filePath, buffer);

            return {
                success: true,
                message: 'File uploaded successfully',
                filePath
            }
        } catch (err) {
            console.error('Error uploading file:', err);
            return error(500);
        }
    }
} satisfies Actions;