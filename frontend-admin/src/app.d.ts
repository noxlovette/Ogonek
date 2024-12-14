// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}
		// interface Locals {}
		// interface PageData {}
		// interface PageState {}
		// interface Platform {}

		interface Article {
			id: number;
			title: string;
			content: string;
			processed_html: string;
			created_at: string;
			updated_at: string;
			related_articles: Article[];
			difficulty: number;
			concepts: Concept[];
			tags_display: String[];
			slug: string;
			category: Category;
		}
		interface Meta {
			categories: Category[];
			concepts: Concept[];
			tags: Tag[];
			articles: Article[];
	}
	interface Category {
		id: string;
		name: string;
		description: string;
	}
	interface Concept {
		id: string;
		name: string;
	}
	interface Tag {
		id?: string;
		name: string;
	}
}
}

export {};
