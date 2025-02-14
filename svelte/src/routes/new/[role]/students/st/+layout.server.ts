import type { LayoutServerLoad } from './$types';
import { error } from '@sveltejs/kit';
import type { Student } from '$lib/types';
import { parseMarkdown } from '$lib/utils';

export const load: LayoutServerLoad = async ({ params, fetch }) => {
	const { id } = params;
	try {
		const response = await fetch(`/axum/student/${id}`);
		const student: Student = await response.json();

		const rendered = await parseMarkdown(student.markdown);

		return {
			student,
			rendered
		};
	} catch (err) {
		throw err;
	}
};
