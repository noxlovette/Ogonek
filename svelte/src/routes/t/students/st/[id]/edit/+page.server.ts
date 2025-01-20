import type { Actions } from '@sveltejs/kit';
import { fail, redirect } from '@sveltejs/kit';

export const actions = {
	update: async ({ request, fetch }) => {
		const formData = await request.formData();
		const markdown = formData.get('markdown');
		const telegramId = formData.get('telegramId');
		const id = formData.get('id');
		let body = {
			markdown,
			telegramId
		};

		const response = await fetch(`/axum/student/${id}`, {
			method: 'PATCH',
			body: JSON.stringify(body)
		});

		if (!response.ok) {
			const errorData = await response.json(); // Parse error details
			console.error('Error updating student:', errorData);
			return {
				success: false,
				error: errorData
			};
		}
		return redirect(303, `/t/students/st/${id}`);
	},
	delete: async ({ request, fetch }) => {
		const formData = await request.formData();
		const id = formData.get('id');
		const response = await fetch(`/axum/student/${id}`, {
			method: 'DELETE'
		});

		if (!response.ok) {
			const errorData = await response.json(); // Parse error details
			console.error('Error deleting student:', errorData);
			return {
				success: false,
				error: errorData
			};
		}

		return redirect(303, '/t/students');
	}
} satisfies Actions;
