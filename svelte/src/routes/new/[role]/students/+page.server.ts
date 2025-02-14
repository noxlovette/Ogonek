import type { Actions, PageServerLoad } from './$types';
import { redirect, fail } from '@sveltejs/kit';


export const load: PageServerLoad = async ({ params }) => {
	if (params.role !== "t") {
		redirect(301, "/unauthorised")
	}
}

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
