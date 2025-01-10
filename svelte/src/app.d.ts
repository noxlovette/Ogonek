// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare global {
	namespace App {
		interface Error {
			message: string;
			errorId: string;
		}
		interface Locals {
			accessToken?: string;
			refreshToken?: string;
			user?: {
				id: string;
				role: string;
			  };
		}
		// interface PageData {}
		// interface PageState {}
		// interface Platform {}
	}
}

export {};
