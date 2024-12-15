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
		due_date: string;
		completed: boolean;
	}

	interface ResponseLogin {
		sessionid: string;
		email: string;
		username: string;
		is_authenticated: boolean;
		success: boolean;
		quizlet_url: string;
		message: string;
	}

	interface User {
		id: string;
		email: string;
		username: string;
		is_authenticated: boolean;
		csrfToken: string;
	}

	interface Lesson {
		id: string;
		title: string;
		content: string;
		created_at: string;
		updated_at: string;
		topic: string;
		bookmarked: boolean;
	}
}
}

export {};
