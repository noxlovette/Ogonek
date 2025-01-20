import type { LayoutServerLoad } from './$types';
import { env } from '$env/dynamic/private';
import type { Lesson, Task } from '$lib/types';

export const load: LayoutServerLoad = async ({ fetch }) => {
	const [lessons, tasks] = await Promise.all([
		fetch('/axum/lesson').then((res) => res.json() as Promise<Lesson[]>),
		fetch('/axum/task').then((res) => res.json() as Promise<Task[]>)
	]);

	return { lessons, tasks };

	// let word = await fetch('https://wordsapiv1.p.rapidapi.com/words?random=true', {
	//	method: 'GET',
	//	headers: {
	//		'x-rapidapi-host': 'wordsapiv1.p.rapidapi.com',
	//		'x-rapidapi-key': `${env.API_WORD_KEY}`,
	//	}
	//}).then((res) => res.json());
};
