import type { Actions } from '@sveltejs/kit';
import { fail, redirect, error } from '@sveltejs/kit';
import { notifyTelegram } from '$lib/server/telegram';

import { Buffer } from 'node:buffer';

import { createHash, randomUUID } from 'node:crypto';
import { writeFile } from 'node:fs/promises';
import path from 'node:path';
import { env } from '$env/dynamic/private';

const MAX_FILE_SIZE = 5 * 1024 * 1024;
const ALLOWED_MIMES = new Set([
	'image/jpeg',
	'image/png',
	'application/pdf',
	'application/zip',        // standard ZIP
	'application/x-zip',      // older ZIP variant
	'application/x-zip-compressed', // some systems use this
	'multipart/x-zip'         // less common but good to support
]);
const encodeFileName = (fileName: string): string => {
	return Buffer.from(fileName).toString('base64url');
};

export const actions = {
	update: async ({ request, fetch }) => {
		const formData = await request.formData();
		const markdown = formData.get('markdown')?.toString() || '';
		const title = formData.get('title')?.toString() || '';
		const id = formData.get('id')?.toString() || '';
		const dueDate = formData.get('dueDate')?.toString() || '';
		const completed = formData.has('completed');

		const assigneeData = formData.get('student')?.toString() || '{}';
		const { assignee = '', telegramId = '' } = JSON.parse(assigneeData);
		const dueDateWithTime =
			dueDate && dueDate !== '' ? new Date(`${dueDate}T23:59:59`).toISOString() : null;

		let body = {
			id,
			title,
			markdown,
			assignee,
			dueDate: dueDateWithTime,
			completed
		};

		const response = await fetch(`/axum/task/t/${formData.get('id')}`, {
			method: 'PATCH',
			body: JSON.stringify(body)
		});

		if (!response.ok) {
			const errorData = await response.json();
			console.error('Error updating task:', errorData);
			return {
				success: false,
				error: errorData
			};
		}

		const message = `You have a new task on Firelight`;

		if (telegramId) {
			const telegramResponse = await notifyTelegram(message, telegramId);
			if (!telegramResponse.ok) {
				return fail(400);
			}
		}

		return redirect(303, `/t/tasks/t/${id}`);
	},
	delete: async ({ request, fetch }) => {
		const formData = await request.formData();
		const id = formData.get('id');
		const response = await fetch(`/axum/task/t/${id}`, {
			method: 'DELETE'
		});

		if (!response.ok) {
			const errorData = await response.json(); // Parse error details
			console.error('Error deleting task:', errorData);
			return {
				success: false,
				error: errorData
			};
		}

		redirect(303, '/t/tasks');
	},
	upload: async ({ request, fetch }) => {
		try {
			const formData = await request.formData();
			const file = formData.get('file');
			const id = formData.get('id');

			// Your existing validations
			if (!file || !(file instanceof Blob)) {
				return fail(400, { message: 'No file provided or invalid file type' });
			}
			if (file.size > MAX_FILE_SIZE) {
				return fail(400, { message: 'File too large, max 5MB allowed' });
			}
			if (!ALLOWED_MIMES.has(file.type)) {
				return fail(400, { message: 'Unsupported file type' });
			}

			const fileId = randomUUID();
			const encodedName = encodeFileName(file.name);

			// Create a new FormData for the nginx upload
			const uploadFormData = new FormData();
			uploadFormData.append('file', file, encodedName);

			// Send to your nginx upload endpoint
			const uploadResponse = await fetch('https://media-staging.noxlovette.com/uploads/', {
				method: 'POST',
				body: uploadFormData
			});

			if (!uploadResponse.ok) {
				return fail(500, { message: 'Failed to upload to storage server' });
			}

			// Get the file URL from the upload response
			const fileUrl = `https://media-staging.noxlovette.com/uploads/${encodedName}`;

			// Update your task with the file URL instead of local path
			let taskResponse = await fetch(`/axum/task/t/${id}`, {
				method: 'PATCH',
				body: JSON.stringify({ filePath: fileUrl })
			});

			if (!taskResponse.ok) {
				return fail(500, { message: await taskResponse.text() });
			}

			return {
				success: true,
				message: 'File uploaded successfully',
				fileId,
				originalName: file.name,
				url: fileUrl
			};

		} catch (err) {
			console.error('Error uploading file:', err);
			return error(500);
		}
	}
} satisfies Actions;
