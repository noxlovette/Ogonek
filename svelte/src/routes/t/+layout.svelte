<script lang="ts">
	import { Dashboard, Todo, Lessons, Quizlet, Settings, Students, Sidebar } from '$lib/components';
	import { setContext } from 'svelte';
	import { lessonStore, taskStore } from '$lib/stores';
	import type { Lesson, Task, Student } from '$lib/types';

	interface Props {
		data: {
			tasks: Task[];
			lessons: Lesson[];
			students: Student[];
		};
		children?: import('svelte').Snippet;
	}

	let { data, children }: Props = $props();

	const { tasks, lessons, students } = data;

	lessonStore.setLessons(lessons);
	taskStore.setTasks(tasks);
	setContext('students', students);
</script>

<Sidebar class="" elements={[Dashboard, Todo, Lessons, Students, Quizlet, Settings]} />

<div
	class="flex flex-col flex-1 px-8 py-4 ml-4 gap-4 overflow-y-auto rounded-lg ring-2 ring-milk-200 bg-white"
>
	{@render children?.()}
</div>
