import type { Actions } from './$types';
import { error, redirect, fail } from '@sveltejs/kit';

const DJANGO_URL = 'http://backend:8000';
const API_KEY_DJANGO = process.env.API_KEY_DJANGO || '';

export const actions: Actions = {
	login: async ({ request, cookies, url }) => {
		const data = await request.formData();
		const username = data.get('username') as string;
		const password = data.get('password') as string;

		const response = await fetch(`${DJANGO_URL}/api/login/`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/x-www-form-urlencoded',
				'X-API-Key': API_KEY_DJANGO
			},
			body: new URLSearchParams({
				username,
				password
			})
		});

		const result: App.ResponseLogin = await response.json();

		if (response.ok) {
			// Set the session cookie with the new sessionid
			cookies.set('sessionid', result.sessionid, {
				path: '/'
			});

			// Check for a redirect parameter from the URL
			const redirectTo = url.searchParams.get('redirectTo') || '/u/dashboard';

			// Redirect to the specified path or default to home
			throw redirect(302, redirectTo);
		} else {
			// Check for specific error messages from the server
			if (result.message?.toLowerCase().includes('password')) {
				return fail(400, { username, incorrect: true, message: result.message });
			} else {
				return fail(400, { success: false, message: result.message || 'Login failed' });
			}
		}
	}
};
