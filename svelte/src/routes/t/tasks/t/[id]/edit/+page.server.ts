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
	upload: async ({ request }) => {
		try {
			const formData = await request.formData();
			const file = formData.get('file');
			const id = formData.get('id');


			console.log('FormData entries:', Array.from(formData.entries()));

			const newFormData = new FormData();
			newFormData.append('file', file, file.name); // Attach the file with its filename
			console.log('new entries:', Array.from(newFormData.entries()));

			if (!file) throw new Error('yikes, no file');

			// Straight to Axum - no extra steps
			const uploadResponse = await fetch('http://upload-service:3001/upload', {
				method: 'POST',
				//	headers: {
				//		'Content-Type': 'multipart/form-data', // Ensure the server expects this
				//	},
				body: newFormData // Send the original FormData - Axum can handle multipart
			});

			if (!uploadResponse.ok) {
				throw new Error('upload failed fam');
			}

			return {
				success: true,
				message: 'uploaded successfully',
			};

		} catch (err) {
			console.error('💀 Upload error:', err);
			return {
				success: false,
				message: err.message
			};
		}
	}
} satisfies Actions;
