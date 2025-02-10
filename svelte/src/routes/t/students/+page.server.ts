import type { Actions } from './$types';
import { redirect, fail } from '@sveltejs/kit';

export const actions: Actions = {
	default: async ({ fetch }) => {
		const response = await fetch('/axum/auth/invite', { method: 'POST' });

		if (!response.ok) {
			return fail(400, { error: 'Failed to generate invite link' });
		}

		const link = await response.json();

		return {
			link
		};
	}
};
