import type { Actions } from './$types';
import { redirect, fail } from '@sveltejs/kit';

export const actions: Actions = {
	default: async ({ request, url }) => {
		const data = await request.formData();
		const username = data.get('username') as string;
		const pass = data.get('password') as string;
		const email = data.get('email') as string;
		const role = data.get('role') as string;
		const name = data.get('name') as string;

		try {
			const response = await fetch('axum/auth/signup', {
				method: 'POST',
				body: JSON.stringify({
					username,
					pass,
					email,
					role,
					name
				})
			});

			throw redirect(302, '/login');
		} catch (error) {
			return fail(400, { success: false, message: error instanceof Error ? error.message : 'Login failed' });
		}
	}
};
