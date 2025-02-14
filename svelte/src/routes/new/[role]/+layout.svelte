<script lang="ts">
	import {
		Dashboard,
		Todo,
		Lessons,
		Quizlet,
		Settings,
		Students,
		Zoom,
		Sidebar,
		WorkArea,
		BottomMenu,
		UsefulLinks, 
		WordOfTheDay
	} from '$lib/components';
	import { lessonStore, studentStore, taskStore } from '$lib/stores';

	import {page } from '$app/state'

	let { data, children } = $props();
	const role = $derived(page.params.role);
	
	let elements = $state([Dashboard, Todo, Lessons, Zoom, Quizlet, Settings]);

	if (role === "t") {
		elements = [Dashboard, Todo, Lessons, Students, Quizlet, Settings]
	}

	const { tasks, lessons, students } = data;

	lessonStore.setLessons(lessons);
	taskStore.setTasks(tasks);
	studentStore.setStudents(students);

</script>

<Sidebar class="text-cacao-50" {elements} />
<WorkArea>
	{@render children?.()}
</WorkArea>
{#if role !== "t"}
<Sidebar
	class="bg-inherit text-inherit ring-transparent shadow-none dark:bg-milk-950 dark:ring-transparent"
	subclass="divide-transparent dark:divide-transparent"
	elements={[UsefulLinks, WordOfTheDay]}
></Sidebar>
{/if}
<BottomMenu {elements} />

<svelte:head>
	<title>Tasks</title>
</svelte:head>