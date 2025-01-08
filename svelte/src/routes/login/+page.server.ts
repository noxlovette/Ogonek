import type { Actions } from './$types';
import { redirect, fail } from '@sveltejs/kit';

const BACKEND_URL = process.env.BACKEND_URL || 'http://localhost:3000';

export const actions: Actions = {
	login: async ({ request, url }) => {
		const data = await request.formData();
		const username = data.get('username') as string;
		const pass = data.get('password') as string;

		const response = await fetch(`${BACKEND_URL}/signin`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
			},
			body: JSON.stringify({
				"username": username,
				"pass": pass,
			})
		});

		const result: App.ResponseLogin = await response.json();

		if (response.ok) {
			console.log("login successful");
			// Check for a redirect parameter from the URL
			const redirectTo = url.searchParams.get('redirectTo') || '/u/dashboard';
			throw redirect(302, redirectTo);
		} else {
			return fail(400, { success: false, message: 'Login failed' });
		}
	}
};
