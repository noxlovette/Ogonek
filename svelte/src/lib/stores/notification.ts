import type { Toast } from '$lib/types';
import { writable } from 'svelte/store';

export const notification = writable<Toast>({
	message: null,
	type: null
});

export function clearNotification() {
	notification.update(() => ({
		message: null,
		type: null
	}));
}
