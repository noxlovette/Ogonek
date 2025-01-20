import type { Actions } from '@sveltejs/kit';
import { fail, redirect, error } from '@sveltejs/kit';
import { notifyTelegram } from '$lib/server/telegram';

import { Buffer } from 'node:buffer';

import { createHash, randomUUID } from 'node:crypto';
import { writeFile } from 'node:fs/promises';
import path from 'node:path';
import { env } from '$env/dynamic/private';

const MAX_FILE_SIZE = 5 * 1024 * 1024;
const ALLOWED_MIMES = new Set(['image/jpeg', 'image/png', 'application/pdf']);
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

		const telegramResponse = await notifyTelegram(message, telegramId);
		if (!telegramResponse.ok) {
			return fail(400);
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

			if (!file || !(file instanceof Blob)) {
				return fail(400, {
					message: 'No file provided or invalid file type'
				});
			}

			if (file.size > MAX_FILE_SIZE) {
				return fail(400, {
					message: 'File too large, max 5MB allowed'
				});
			}

			if (!ALLOWED_MIMES.has(file.type)) {
				return fail(400, {
					message: 'Unsupported file type'
				});
			}
			const fileId = randomUUID();
			const salt = crypto.getRandomValues(new Uint8Array(16));
			const arrayBuffer = await file.arrayBuffer();
			const buffer = Buffer.from(arrayBuffer);

			const hash = createHash('sha256')
				.update(Buffer.concat([buffer, Buffer.from(salt)]))
				.digest('hex');

			const encodedName = encodeFileName(file.name);
			const filePath = path.join(env.UPLOAD_DIR, encodedName);

			await writeFile(filePath, buffer);

			let response = await fetch(`/axum/task/t/${id}`, {
				method: 'PATCH',
				body: JSON.stringify({ filePath })
			});

			if (!response.ok) {
				return fail(500, { message: response.text() });
			}

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
