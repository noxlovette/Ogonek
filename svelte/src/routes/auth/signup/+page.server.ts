import type { Actions } from './$types';
import { redirect, fail } from '@sveltejs/kit';

export const actions: Actions = {
	default: async ({ request, url, fetch }) => {
		const data = await request.formData();
		const username = data.get('username') as string;
		const pass = data.get('password') as string;
		const email = data.get('email') as string;
		const role = data.get('role') as string;
		const name = data.get('name') as string;

		const invite_token = url.searchParams.get('invite')

		const response = await fetch('/axum/auth/signup', {
			method: 'POST',
			body: JSON.stringify({
				username,
				pass,
				email,
				role,
				name
			})
		});

		if (invite_token) {
			const student_id = await response.json().then((data) => data.id);
			const invite = await fetch('/axum/auth/bind', {
				method: 'POST',
				body: JSON.stringify({
					invite_token,
					student_id
				})
			});

			if (!invite.ok) {
				return fail(400, { message: 'Failed to bind invite' });
			}
		}
		if (response.ok) {
			return redirect(302, '/auth/login');
		}

		const { error } = await response.json();
		return fail(400, { message: error });
	}
} satisfies Actions;