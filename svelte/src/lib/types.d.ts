export interface Task {
	id: string;
	title: string;
	markdown: string;
	priority: int;
	createdAt: string;
	updatedAt: string;
	dueDate: string;
	completed: boolean;
	filePath: string;
	createdBy: string;
	assignee: string;
	assigneeName: string;
}
export interface Toast {
	message: string | null;
	type: 'success' | 'error' | 'info' | null;
}
export interface Profile {
	quizletUrl: string | null;
	zoomUrl: string | null;
	bio: string | null;
	avatarUrl: string | null;
}

export interface User {
	name: string | null;
	username: string | null;
	role: string | null;
	email: string | null;
}

export interface Lesson {
	id: string;
	manualDate?: string;
	title: string;
	markdown: string;
	createdAt: string;
	updatedAt: string;
	topic: string;
	bookmarked: boolean;
	highlighted: string;
	assignee: string;
	assigneeName: string;
}

export interface LessonStore {
	title: string;
	markdown: string;
	topic: string;
}

export interface Student {
	id: string;
	name: string;
	username: string;
	email: string;
	role: string;
	markdown: string;
	joined: string;
	telegramId: string;
}

export interface BaseTableItem {
	id: string;
}

export interface TableConfig<T extends BaseTableItem> {
	columns: {
		key: keyof T;
		label: string;
		searchable?: boolean;
		formatter?: (value: T[keyof T]) => string;
	}[];
}

export interface UserData {
	user: User;
	profile: Profile;
}

export interface CookieOptions {
	path?: string;
	httpOnly?: boolean;
	secure?: boolean;
	sameSite?: 'lax' | 'strict' | 'none';
	domain?: string;
	maxAge?: number;
}
