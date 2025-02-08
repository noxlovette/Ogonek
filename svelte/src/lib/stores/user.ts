import { writable } from 'svelte/store';
import type { User, Profile } from '$lib/types';

export const initialUser: User = {
	username: '',
	name: '',
	role: '',
	email: ''
};

export const initialProfile: Profile = {
	quizletUrl: null,
	zoomUrl: null,
	bio: null,
	avatarUrl: null
};

export const profile = writable<Profile>(initialProfile);
export const user = writable<User>(initialUser);
export function setProfile(data: Profile) {
	profile.update((currentState) => ({
		...currentState,
		...data
	}));
}
export function setUser(data: User) {
	user.update((currentState) => ({
		...currentState,
		...data
	}));
}

export function clearUser() {
	user.update(() => ({
		username: '',
		name: '',
		role: '',
		email: ''
	}));
}
