<script lang="ts">
	import Header from '$lib/components/fonts/Header.svelte';
	import Search from '$lib/components/Search.svelte';
	import { user, language, translations } from '$lib/stores';

	import { formatDate } from '$lib/utils';

	import { getGreeting } from '$lib/utils';

	import { getContext } from 'svelte';

	let hidden = false;
	const greeting = getGreeting();

	let tasks: App.Task[] = getContext('tasks');
	let lessons: App.Lesson[] = getContext('lessons');
	let filtered = tasks.filter((task) => task.completed === false);
	let recent_tasks = filtered.slice(0, 3);
	let recent_lesson = lessons[0];
</script>

<svelte:head>
	<title>Dashboard</title>
</svelte:head>

<div class="flex flex-col items-center">
	<h1 class="text-xl lg:text-2xl xl:text-3xl font-serif">
		{$translations[greeting][$language]}, {$user.first_name}
	</h1>
</div>

<div
	class="w-full items-center justify-center flex flex-col my-8 border-2 py-4 border-sand-900/40 rounded-lg"
>
	<Header>
		{$translations.recent_activity[$language]}
	</Header>
	<div class="grid gap-4 grid-cols-1 md:grid-cols-2 w-full my-4">
		<div class="flex flex-col items-center space-y-2">
			<h2 class="text-2xl">
				{$translations.new_tasks[$language]}
			</h2>
			{#each recent_tasks as task}
				<a
					href="/u/tasks/"
					class="px-1 py-2 inline-flex w-1/2 justify-center bg-sand-900/10 rounded-lg"
				>
					{task.title}
				</a>
			{/each}
		</div>
		<div class="flex flex-col items-center space-y-2">
			<h2 class="text-2xl">
				{$translations.new_lesson[$language]}
			</h2>
			<a
				href="/u/lessons/l/{recent_lesson.id}"
				class="px-1 py-2 inline-flex w-1/2 justify-center bg-sand-900/10 rounded-lg"
			>
				{formatDate(recent_lesson.manual_date || recent_lesson.created_at)}
			</a>
		</div>
	</div>
</div>
<Search bind:hidden />

<style>
	a:hover {
		@apply bg-sand-900/60 transition-colors duration-300  text-sand-100;
	}
</style>
