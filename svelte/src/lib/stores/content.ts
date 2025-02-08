import { writable } from 'svelte/store';
import type { Lesson, Task } from '$lib/types';

const createLessonStore = () => {
	const { subscribe, set, update } = writable<Lesson[]>([]);

	return {
		subscribe,
		setLessons: (lessons: Lesson[]) => set(lessons),
		addLesson: (lesson: Lesson) => update((lessons) => [...lessons, lesson]),
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

export const tableQuery = writable('');
