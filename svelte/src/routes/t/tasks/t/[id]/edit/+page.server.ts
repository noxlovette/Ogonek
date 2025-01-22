import type { Actions } from '@sveltejs/kit';
import { fail, redirect } from '@sveltejs/kit';
import { notifyTelegram } from '$lib/server/telegram';
import { env } from '$env/dynamic/private';

export const actions = {
	update: async ({ request, fetch }) => {
		const formData = await request.formData();
		const markdown = formData.get('markdown')?.toString() || '';
		const title = formData.get('title')?.toString() || '';
		const id = formData.get('id')?.toString() || '';
		const dueDate = formData.get('dueDate')?.toString() || '';
		const completed = formData.has('completed');
		const filePath = formData.get('filePath')?.toString() || '';

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
			completed,
			filePath
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

		const message = `You have a new task: "${title}"\\. You can view it on [Firelight](https://staging\\.noxlovette\\.com/s/tasks)\\.`;


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
			const file = formData.get('file') as File;

			if (!file) throw new Error('yikes, no file');

			const uploadResponse = await fetch('/file-server/upload', {
				method: 'POST',
				body: formData
			});

			if (!uploadResponse.ok) {
				const { error } = await uploadResponse.json()
				return fail(500, { message: error })
			}

			const filePath = (await uploadResponse.text()).replace(/^"|"$/g, '');

			return {
				success: true,
				filePath,
				message: 'Uploaded successfully',
			};

		} catch (err) {
			console.error('💀 Upload error:', err);
			return fail(500, { message: err })
		}
	}
} satisfies Actions;
