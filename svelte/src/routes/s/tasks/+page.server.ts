import type { Actions } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { error, fail } from '@sveltejs/kit';

export const actions = {
	default: async ({ request, fetch }) => {

		const formData = await request.formData();
		const id = formData.get('id')
		let body = {
			completed: formData.has('completed'),
			id
		};

		const response = await fetch(`/axum/task/t/${id}`, {
			method: 'PATCH',
			body: JSON.stringify(body)
		});



		const { error } = await response.json()


		if (!response.ok) {
			return fail(500, { message: "Something's off" })
		}

		return {
			success: true,
		};
	},
} satisfies Actions;
