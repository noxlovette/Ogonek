import type { Actions } from './$types';
import { redirect, fail } from '@sveltejs/kit';
import { APIClient } from '$lib/server/api';

export const actions: Actions = {
	default: async ({ request, url, cookies }) => {
		const data = await request.formData();
		const username = data.get('username') as string;
		const pass = data.get('password') as string;

		const api = APIClient.getInstance();

		const response = await api.request('/auth/signin', {
			method: 'POST',
			body: JSON.stringify({
				"username": username,
				"pass": pass,
			})
		});

		if (response.ok) {
			const rCookies = response.headers.getSetCookie?.() || [];
			console.log('All headers:', [...response.headers.entries()]);
			console.log('Cookies:', rCookies);
			const refreshToken = rCookies.find(rCookie => rCookie.startsWith('refresh-token='))?.split(';')[0];
			
			const accessToken = response.headers.get('authorization')?.replace('Bearer ', '');

			if (refreshToken) {
				cookies.set('refresh-token', refreshToken, {  // This cookies should come from your function parameters
					path: '/',
				});
			}
			if (accessToken) {
				cookies.set("access-token", accessToken, {
					path: '/',
				});
			}


			// const redirectTo = url.searchParams.get('redirectTo') ?? '/u/dashboard';
			
			return {
				success: true,
				// redirectTo
			};
		} else {
			return fail(400, { success: false, message: 'Login failed' });
		}
	}
};
