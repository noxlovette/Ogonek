import type { ClientInit } from '@sveltejs/kit';
import { setProfile, setUser } from '$lib/stores';


export const init: ClientInit = async () => {
	const user = localStorage.getItem('user') || '';
	const profile = localStorage.getItem('profile') || '';
	
	if (user) {
		setUser(JSON.parse(user));
		setProfile(JSON.parse(profile));
	}
};
