<script lang="ts">
	import {
		Dashboard,
		Todo,
		Lessons,
		Quizlet,
		Settings,
		Students,
		Sidebar,
		WorkArea,
		BottomMenu
	} from '$lib/components';
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
<WorkArea>
	{@render children?.()}
</WorkArea>
<BottomMenu elements={[Dashboard, Todo, Lessons, Students, Quizlet, Settings]} />
