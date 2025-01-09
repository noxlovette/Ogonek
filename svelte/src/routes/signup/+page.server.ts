import type { Actions } from './$types';
import { redirect, fail } from '@sveltejs/kit';
import { APIClient } from '$lib/server/api';

export const actions: Actions = {
	default: async ({ request, url }) => {
		const data = await request.formData();
		const username = data.get('username') as string;
		const pass = data.get('password') as string;
		const email = data.get('email') as string;
		const role = data.get('role') as string;
		const name = data.get('name') as string;

		const api = APIClient.getInstance();

		try {

			const response = await api.request('/auth/signup', {
				method: 'POST',
				body: JSON.stringify({
					"username": username,
					"pass": pass,
					"email": email,
					"role": role,
					"name": name
				})
			});
			
				console.log(response);
				throw redirect(302, '/login');
			} catch (error) {
				console.error(error);
				return fail(400, { success: false, message: 'Login failed' });
			}
		}
	};
