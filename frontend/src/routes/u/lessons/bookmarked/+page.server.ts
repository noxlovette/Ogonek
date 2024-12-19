import type { PageServerLoad } from './$types';

const VITE_API_URL = 'http://backend:8000';

export const load: PageServerLoad = async ({ params, cookies }) => {
	const sessionid = cookies.get('sessionid');
	const csrfToken = cookies.get('csrftoken') || '';

	try {
		const lessons = await fetch(`${VITE_API_URL}/api/lessons/`, {
			headers: {
				Cookie: `sessionid=${sessionid}; csrftoken=${csrfToken}`,
				'X-API-Key': process.env.API_KEY_DJANGO || ''
			}
		}).then((res) => res.json());

		return {
			lessons
		};
	} catch (err) {
		// If any error occurs, propagate it to SvelteKit's error handling
		throw err;
	}
};
