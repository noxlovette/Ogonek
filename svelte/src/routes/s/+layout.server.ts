import type { LayoutServerLoad } from '../$types';
import { env} from '$env/dynamic/private';

export const load: LayoutServerLoad = async ({fetch}) => {

		let lessons = await fetch("/axum/lesson").then((res) => res.json());
		
		// let word = await fetch('https://wordsapiv1.p.rapidapi.com/words?random=true', {
		//	method: 'GET',
		//	headers: {
		//		'x-rapidapi-host': 'wordsapiv1.p.rapidapi.com',
		//		'x-rapidapi-key': `${env.API_WORD_KEY}`,
		//	}
		//}).then((res) => res.json());

	return { lessons };
};
