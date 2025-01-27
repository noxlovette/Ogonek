import { fail, type Actions } from '@sveltejs/kit';
import { ValidateAccess } from '$lib/utils';
import { env } from '$env/dynamic/private';
import { parseCookieOptions } from '$lib/utils';

export const actions: Actions = {
	default: async ({ request, fetch, cookies }) => {
		try {
			const data = await request.formData();
			const username = data.get('username') as string;
			const pass = data.get('password') as string;

			if (!username || !pass) {
				return fail(400, {
					message: 'Missing required fields'
				});
			}

			const response = await fetch('/axum/auth/signin', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({ username, pass })
			});

			if (!response.ok) {
				const { error } = await response.json();
				return fail(400, { message: error });
			}

			// Handle cookies
			response.headers.getSetCookie().forEach((cookie) => {
				const [fullCookie, ...opts] = cookie.split(';');
				const [name, value] = fullCookie.split('=');

				const cookieOptions = parseCookieOptions(opts);
				cookies.set(name, value, cookieOptions);
			});

			const { accessToken } = await response.json();
			const user = await ValidateAccess(accessToken);

			if (!user) {
				return fail(401, {
					message: 'Invalid access token'
				});
			}

			const profileResponse = await fetch('/axum/profile');
			if (!profileResponse.ok) {
				return fail(500, {
					message: 'Failed to fetch profile'
				});
			}

			const profile = await profileResponse.json();

			return {
				success: true,
				message: 'Login successful',
				user,
				profile
			};
		} catch (error) {
			console.error('Signin error:', error);
			return fail(500, {
				message: error instanceof Error ? error.message : 'Internal server error'
			});
		}
	}
};
