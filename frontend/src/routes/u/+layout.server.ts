import { redirect } from '@sveltejs/kit';
import type { LayoutServerLoad } from '../$types';

const BACKEND_URL = process.env.BACKEND_URL || 'http://backend:8000';
const VITE_API_WORD_KEY = process.env.VITE_API_WORD_KEY;
const API_KEY_DJANGO = process.env.API_KEY_DJANGO || '';

export const load: LayoutServerLoad = async () => {

		let lessons = await fetch(`${BACKEND_URL}/lessons`).then((res) => res.json());


		let word = await fetch('https://wordsapiv1.p.rapidapi.com/words?random=true', {
			method: 'GET',
			headers: {
				'x-rapidapi-host': 'wordsapiv1.p.rapidapi.com',
				'x-rapidapi-key': `${VITE_API_WORD_KEY}` // Replace this with your actual RapidAPI key
			}
		}).then((res) => res.json());

	return { word, lessons };
};
