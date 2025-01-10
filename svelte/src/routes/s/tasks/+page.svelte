<script lang="ts">
	import TaskBox from '$lib/components/task/TaskBox.svelte';
	import { translations, language } from '$lib/stores';

	import { getContext } from 'svelte';

	import { Bird } from 'lucide-svelte';
	import Header from '$lib/components/fonts/Header.svelte';

	let tasks: App.Task[] = getContext('tasks');

	let filtered = tasks.filter((task) => task.completed === false);
	let completed = tasks.filter((task) => task.completed === true);

	let completedVisible = $state(false);
</script>

<svelte:head>
	<title>Tasks</title>
</svelte:head>

{#if filtered.length > 0}
	<Header>
		{$translations.tasks[$language]}
	</Header>
	<div class="grid grid-cols-1 gap-4 lg:grid-cols-2 my-4 w-full">
		{#each filtered as task}
			<TaskBox {task} />
		{/each}
	</div>
{:else}
	<Bird class="size-32 mx-auto text-sand-900" />
	<p>
		{$translations.no_tasks[$language]}
	</p>
{/if}
<button
	onclick={() => (completedVisible = !completedVisible)}
	class="text-sm font-sans text-left hover:text-sand-800"
>
	{#if completedVisible}
		{$translations.tasks_notcompleted[$language]}
	{:else}
		{$translations.tasks_completed[$language]}
	{/if}
</button>

{#if completedVisible}
	<div class="grid grid-cols-2 my-4 gap-4">
		{#each completed as task}
			<TaskBox {task} />
		{/each}
	</div>
{/if}
