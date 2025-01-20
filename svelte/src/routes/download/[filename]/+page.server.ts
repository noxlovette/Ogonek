// +page.server.ts
import { env } from '$env/dynamic/private';
import fs from 'fs/promises';
import path from 'path';

export async function load({ params }) {
	const fullFilePath = decodeURIComponent(params.filename);
	const filename = path.basename(fullFilePath);
	const filePath = path.join(env.UPLOAD_DIR, filename);

	try {
		const fileBuffer = await fs.readFile(filePath);
		const decodedFileName = Buffer.from(filename, 'base64').toString('utf-8');

		return {
			filename: decodedFileName,
			headers: {
				'Content-Type': 'application/octet-stream',
				'Content-Disposition': `attachment; filename="${decodedFileName}"`
			},
			body: fileBuffer
		};
	} catch (error) {
		console.error('Error reading file:', error);
		throw new Error('Failed to load the requested file.');
	}
}
