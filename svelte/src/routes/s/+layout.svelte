<script lang="ts">
	import Sidebar from '$lib/components/sidebar/Sidebar.svelte';
	import { setContext } from 'svelte';
	import BottomMenu from '$lib/components/mobile/BottomMenu.svelte';
	import type { Lesson, Task } from '$lib/types';

	interface Props {
		data: any;
		children?: import('svelte').Snippet;
	}

	let { data, children }: Props = $props();

	const tasks: Task[] = data.tasks;
	const lessons: Lesson[] = data.lessons;

	setContext('tasks', tasks);
	setContext('lessons', lessons);
	// setContext('word', data.word);

	import {
		Dashboard,
		Todo,
		Lessons,
		Zoom,
		Quizlet,
		Settings
	} from '$lib/components/sidebar/elements';
	import { RecentLessons, UsefulLinks, WordOfTheDay } from '$lib/components/sidebar/groups';
</script>

<Sidebar class="" elements={[Dashboard, Todo, Lessons, Zoom, Quizlet, Settings]} />
<div class="flex flex-col justify-start items-center font-medium overflow-auto flex-1">
	<div class="flex flex-1 flex-col size-full px-8 py-4">
		{@render children?.()}
	</div>
	<BottomMenu />
</div>
<Sidebar
	class="bg-inherit text-inherit px-0 py-0 ring-0 shadow-none"
	elements={[UsefulLinks, WordOfTheDay, RecentLessons]}
></Sidebar>
