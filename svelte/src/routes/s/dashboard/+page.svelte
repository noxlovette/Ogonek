<script lang="ts">
	import Header from '$lib/components/fonts/Header.svelte';
	import Search from '$lib/components/UI/Search.svelte';
	import { user, language, translations } from '$lib/stores';

	import { formatDate } from '$lib/utils';

	import { getGreeting } from '$lib/utils';

	import { getContext } from 'svelte';

	let hidden = $state(false);
	const greeting = getGreeting();
	import type { Lesson } from '$lib/types';

	// let tasks: App.Task[] = getContext('tasks');
	let lessons: Lesson[] = getContext('lessons');
	// let filtered = tasks.filter((task) => task.completed === false);
	// let recent_tasks = filtered.slice(0, 3);
	// let recent_lesson = lessons[0];
</script>

<svelte:head>
	<title>Dashboard</title>
</svelte:head>

<div class="w-full items-center justify-center flex flex-col">
	<Header>Recent Activity</Header>
	<div class="grid gap-4 grid-cols-1 md:grid-cols-2 w-full my-4">
		<div class="flex flex-col items-center space-y-2">
			<h2 class="text-2xl">New Tasks</h2>
		</div>
		<div class="flex flex-col items-center space-y-2">
			<h2 class="text-2xl">New Lessons</h2>
		</div>
	</div>
</div>
<Search bind:hidden />

<style>
	a:hover {
		@apply bg-sand-900/60 transition-colors duration-300  text-sand-100;
	}
</style>
