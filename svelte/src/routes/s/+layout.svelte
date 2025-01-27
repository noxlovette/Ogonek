<script lang="ts">
	import { BottomMenu, Sidebar, WorkArea } from '$lib/components';
	import type { Lesson, Task } from '$lib/types';
	import { lessonStore, taskStore } from '$lib/stores';

	let { data, children } = $props();

	const tasks: Task[] = data.tasks;
	const lessons: Lesson[] = data.lessons;

	lessonStore.setLessons(lessons);
	taskStore.setTasks(tasks);

	import {
		Dashboard,
		Todo,
		Lessons,
		Zoom,
		Quizlet,
		Settings
	} from '$lib/components/sidebar/elements';
	import { UsefulLinks, WordOfTheDay } from '$lib/components/sidebar/groups';
</script>

<Sidebar class="" elements={[Dashboard, Todo, Lessons, Zoom, Quizlet, Settings]} />
<WorkArea>
	{@render children?.()}
</WorkArea>
<Sidebar
	class="bg-inherit text-inherit ring-transparent shadow-none dark:bg-milk-950 dark:ring-transparent"
	subclass="divide-transparent dark:divide-transparent"
	elements={[UsefulLinks, WordOfTheDay]}
></Sidebar>
<BottomMenu elements={[Dashboard, Todo, Lessons, Zoom, Quizlet, Settings]} />
