import type { Actions } from '@sveltejs/kit';
import { fail } from '@sveltejs/kit';

export const actions = {
	default: async ({ request, fetch }) => {
		const formData = await request.formData();
		const id = formData.get('id');
		const completed = formData.get('completed') === 'true';
		const body = {
			completed,
			id
		};
		console.log(formData.has('completed'));
		const response = await fetch(`/axum/task/t/${id}`, {
			method: 'PATCH',
			body: JSON.stringify(body)
		});

		const { error } = await response.json();

		if (!response.ok) {
			return fail(500, { message: error || "Something's off" });
		}

		return {
			success: true
		};
	}
} satisfies Actions;
