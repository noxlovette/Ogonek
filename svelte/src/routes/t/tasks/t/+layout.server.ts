import type { LayoutServerLoad } from './$types';
import { error } from '@sveltejs/kit';
import type { Task } from '$lib/types';
import { parseMarkdown } from '$lib/utils';


export const load: LayoutServerLoad = async ({ params, fetch }) => {

	const { id } = params;
	try {
		const response = await fetch(`/axum/task/t/${id}`);
		const task: Task = await response.json();

		const rendered = await parseMarkdown(task.markdown);

		return {
			task,
			rendered
		};
	} catch (err) {
		throw err;
	}
};
