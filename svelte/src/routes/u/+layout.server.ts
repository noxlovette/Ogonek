import { redirect } from '@sveltejs/kit';
import type { LayoutServerLoad } from '../$types';


const API_WORD_KEY = process.env.VITE_API_WORD_KEY;
const BACKEND_URL=import.meta.env.VITE_BACKEND_URL;
const API_KEY_AXUM = process.env.VITE_API_KEY_AXUM;


export const load: LayoutServerLoad = async () => {
	// const BACKEND_URL = "http://localhost:3000";
	console.log(API_KEY_AXUM)
		let lessons = await fetch(`${BACKEND_URL}/lessons`).then((res) => res.json());


		let word = await fetch('https://wordsapiv1.p.rapidapi.com/words?random=true', {
			method: 'GET',
			headers: {
				'x-rapidapi-host': 'wordsapiv1.p.rapidapi.com',
				'x-rapidapi-key': `${API_WORD_KEY}`,
				'x-api-key': `${API_KEY_AXUM}`
			}
		}).then((res) => res.json());

	return { word, lessons };
};
