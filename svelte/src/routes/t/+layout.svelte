<script lang="ts">
	import { Dashboard, Todo, Lessons, Quizlet, Settings, Students, Sidebar } from '$lib/components';
	import { setContext } from 'svelte';
	import { lessonsStore } from '$lib/stores';
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

	lessonsStore.setLessons(lessons);

	setContext('tasks', tasks);
	setContext('lessons', lessons);
	setContext('students', students);
</script>

<Sidebar class="" elements={[Dashboard, Todo, Lessons, Students, Quizlet, Settings]} />
<div class="flex flex-col justify-start items-center font-medium overflow-auto flex-1">
	<div class="flex flex-1 flex-col size-full px-8 py-4 gap-4">
		{@render children?.()}
	</div>
</div>
