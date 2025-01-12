<script lang="ts">
	import Sidebar from '$lib/components/sidebar/Sidebar.svelte';
	import { setContext } from 'svelte';
	import BottomMenu from '$lib/components/mobile/BottomMenu.svelte';
	import type { Lesson } from '$lib/types';

	interface Props {
		data: any;
		children?: import('svelte').Snippet;
	}

	let { data, children }: Props = $props();

	// const tasks: App.Task[] = data.tasks;
	const lessons: Lesson[] = data.lessons;

	// setContext('tasks', tasks);
	setContext('lessons', lessons);

	import {
		Dashboard,
		Todo,
		Lessons,
		Zoom,
		Quizlet,
		Settings,

		Students

	} from '$lib/components/sidebar/elements';
	import { RecentLessons, UsefulLinks, WordOfTheDay } from '$lib/components/sidebar/groups';
</script>

<Sidebar class="" elements={[Dashboard, Todo, Lessons, Students, Quizlet, Settings]} />
<div class="flex flex-col justify-start items-center font-medium overflow-auto flex-1">
	<div class="flex flex-1 flex-col size-full px-8 py-4">
		{@render children?.()}
	</div>
	<BottomMenu />
</div>
