import { redirect, type Actions } from '@sveltejs/kit';

export const actions: Actions = {
	new: async ({ fetch }) => {
		const body = {
			title: 'New Task',
			markdown: '## Try adding some content here'
		};

		const response = await fetch(`/axum/task`, {
			method: 'POST',
			body: JSON.stringify(body)
		});

		const { id } = await response.json();

		if (response.ok) {
			return redirect(301, `/t/tasks/t/${id}/edit`);
		}
	}
};
