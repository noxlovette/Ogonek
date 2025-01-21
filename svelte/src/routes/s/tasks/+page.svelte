<script lang="ts">
	import type { PageData } from './$types';
	import { slide } from 'svelte/transition';
	import { H2, TaskCard } from '$lib/components';

	let { data }: { data: PageData } = $props();
	const { tasks } = data;

	let pending = tasks.filter((task) => !task.completed);
	let completed = tasks.filter((task) => task.completed);
	let completedVisible = $state(false);

	$effect(() => {
		pending.sort((a, b) => new Date(b.updatedAt).getTime() - new Date(a.updatedAt).getTime());
		completed.sort((a, b) => new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime());
	});
</script>

<svelte:head>
	<title>Tasks</title>
</svelte:head>
<!-- Active Tasks Section -->
<section class="space-y-4">
	<H2>Active Tasks ({pending.length})</H2>
	<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
		{#each pending as task (task.id)}
			<TaskCard {task} interactive={true} />
		{/each}
	</div>
</section>

<!-- Completed Tasks Toggle & Section -->
<section class="space-y-4">
	<button
		class="inline-flex items-center space-x-2 text-milk-700 hover:text-milk-900 transition-colors"
		onclick={() => (completedVisible = !completedVisible)}
	>
		<span class="text-lg font-medium">Completed Tasks ({completed.length})</span>
		<svg
			class="w-5 h-5 transform transition-transform duration-200"
			class:rotate-180={completedVisible}
			viewBox="0 0 20 20"
			fill="currentColor"
		>
			<path
				d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z"
			/>
		</svg>
	</button>

	{#if completedVisible}
		<div
			transition:slide={{ duration: 300 }}
			class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6"
		>
			{#each completed as task (task.id)}
				<TaskCard {task} interactive />
			{/each}
		</div>
	{/if}
</section>
