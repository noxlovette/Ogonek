import type { Actions } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { error } from '@sveltejs/kit';

const BACKEND_URL = process.env.BACKEND_URL || 'http://backend:8000';

export const load: PageServerLoad = async ({ params, cookies }) => {
	const { slug } = params;
	const sessionid = cookies.get('sessionid');
	const csrfToken = cookies.get('csrftoken') || '';
	const headers = {
		Cookie: `sessionid=${sessionid}; csrftoken=${csrfToken}`,
		'X-API-Key': process.env.API_KEY_DJANGO || ''
	};

	try {
		const response = await fetch(`${BACKEND_URL}/api/lessons/${slug}/`, {
			headers: headers
		});

		if (!response.ok) {
			// Check for unauthorized access or similar errors
			if (response.status === 403 || response.status === 401) {
				throw error(403, 'You do not have permission to access this lesson.');
			}
			// For other errors, you might want to handle them differently
			const errorData = await response.json();
			throw error(response.status, errorData.message || 'Error fetching lesson');
		} 

		const lesson = await response.json();

		return {
			lesson
		};
	} catch (err) {
		// If any error occurs, propagate it to SvelteKit's error handling
		throw err;
	}
};

export const actions = {
	bookmark: async ({ request, cookies }) => {
		const formData = await request.formData();
		const sessionid = cookies.get('sessionid');
		const csrfToken = cookies.get('csrftoken');

		let body = {
			id: formData.get('id'),
			bookmarked: formData.get('bookmarked')
		};

		try {
			const response = await fetch(`${BACKEND_URL}/api/lessons/${formData.get('id')}/`, {
				method: 'PATCH',
				headers: {
					'Content-Type': 'application/json',
					Cookie: `sessionid=${sessionid}; csrftoken=${csrfToken}`,
					'X-CSRFToken': csrfToken,
					'X-API-Key': process.env.API_KEY_DJANGO || ''
				},
				body: JSON.stringify(body)
			});

			if (!response.ok) {
				const errorData = await response.json(); // Parse error details
				console.error('Error updating lesson:', errorData);
				return {
					success: false,
					error: errorData
				};
			}

			const lessons = await fetch(`${BACKEND_URL}/api/lessons/`, {
				headers: {
					Cookie: `sessionid=${sessionid}; csrftoken=${csrfToken}`,
					'X-API-Key': process.env.API_KEY_DJANGO || ''
				}
			}).then((res) => res.json());

			return {
				success: true,
				message: 'Lesson updated successfully',
				lessons: lessons // Return the updated list of lessons
			};
		} catch (error) {
			console.error('Fetch error:', error);
			return {
				success: false,
				error: 'An error occurred while updating the lesson'
			};
		}
	}
} satisfies Actions;
