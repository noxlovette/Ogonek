import type { Actions } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { error } from '@sveltejs/kit';
import type { Lesson } from '$lib/types';


export const load: PageServerLoad = async ({ params, fetch }) => {

	const { slug } = params;
	try {
		const response = await fetch(`/axum/lesson/l/${slug}`);

		if (!response.ok) {
			if (response.status === 403 || response.status === 401) {
				throw error(403,
					{
						message: 'You are not authorized to access this lesson. Please log in to continue.',
						errorId: 403
					});
			}
			const errorData = await response.json();
			throw error(response.status, errorData.message || 'Error fetching lesson');
		}

		const lesson: Lesson = await response.json();

		return {
			lesson
		};
	} catch (err) {
		throw err;
	}
};
