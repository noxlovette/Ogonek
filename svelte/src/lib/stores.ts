import { writable, derived } from 'svelte/store';
import type { User, Profile, Toast, Lesson, Task } from './types';

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

const createLessonStore = () => {
	const { subscribe, set, update } = writable<Lesson[]>([]);

	return {
		subscribe,
		// Batch update - more performant than individual updates
		setLessons: (lessons: Lesson[]) => set(lessons),

		// Helper to add single lesson if needed
		addLesson: (lesson: Lesson) => update((lessons) => [...lessons, lesson]),

		// Clear store
		reset: () => set([])
	};
};

export const lessonStore = createLessonStore();

const createTaskStore = () => {
	const { subscribe, set, update } = writable<Task[]>([]);

	return {
		subscribe,

		setTasks: (tasks: Task[]) => set(tasks),
		addTask: (task: Task) => update((tasks) => [...tasks, task]),

		reset: () => set([])
	};
};

export const taskStore = createTaskStore();

export const isSearchOpen = writable(false);
