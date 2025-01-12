import type { LayoutServerLoad } from './$types';
import { error } from '@sveltejs/kit';
import type { Lesson } from '$lib/types';
import { parseMarkdown } from '$lib/utils';


export const load: LayoutServerLoad = async ({ params, fetch }) => {

	const { id } = params;
	try {
		const response = await fetch(`/axum/lesson/l/${id}`);
		const lesson: Lesson = await response.json();

		const rendered = await parseMarkdown(lesson.markdown);

		return {
			lesson,
			rendered
		};
	} catch (err) {
		throw err;
	}
};
