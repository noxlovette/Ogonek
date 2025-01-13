<script lang="ts">
	import Sidebar from '$lib/components/sidebar/Sidebar.svelte';
	import { setContext } from 'svelte';
	import BottomMenu from '$lib/components/mobile/BottomMenu.svelte';
	import type { Lesson, Task, Student } from '$lib/types';

	interface Props {
		data: any;
		children?: import('svelte').Snippet;
	}

	let { data, children }: Props = $props();

	const tasks: Task[] = data.tasks;
	const lessons: Lesson[] = data.lessons;
	const students: Student[] = data.students;

	setContext('tasks', tasks);
	setContext('lessons', lessons);
	setContext('students', students);

	import {
		Dashboard,
		Todo,
		Lessons,
		Quizlet,
		Settings,
		Students
	} from '$lib/components/sidebar/elements';
</script>

<Sidebar class="" elements={[Dashboard, Todo, Lessons, Students, Quizlet, Settings]} />
<div class="flex flex-col justify-start items-center font-medium overflow-auto flex-1">
	<div class="flex flex-1 flex-col size-full px-8 py-4 gap-4">
		{@render children?.()}
	</div>
	<BottomMenu />
</div>
