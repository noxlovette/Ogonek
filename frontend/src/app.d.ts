// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}
		// interface Locals {}
		// interface PageData {}
		// interface PageState {}
		// interface Platform {}

		interface Task {
		id: string;
		title: string;
		content: string;
		priority: int;
		created_at: string;
		updated_at: string;
		status: string;
	}
	}
}

export {};
