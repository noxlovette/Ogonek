import type { PageServerLoad } from './$types';
import { fail, redirect } from '@sveltejs/kit';
import type { Lesson } from '$lib/types';
import { parseMarkdown } from '$lib/utils';


export const load: PageServerLoad = async ({ params, fetch }) => {

	const { id } = params;
	try {
		const response = await fetch(`/axum/lesson/l/${id}`);
		if (!response.ok) {
			throw redirect(303, '/t/lessons/')
		}
		const lesson: Lesson = await response.json();
		const rendered = await parseMarkdown(lesson.markdown);


		return {
			lesson,
			rendered
		};
	} catch (e) {
		console.debug(e)
		throw redirect(303, '/t/lessons/')
	}
};
