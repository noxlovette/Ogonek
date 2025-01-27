<script lang="ts">
	import { H1, H2 } from '$lib/components';
	import { Download, Loader2, CheckSquare, Square } from 'lucide-svelte';
	import { formatDateTime } from '$lib/utils';
	import type { PageData } from './$types';
	import { enhance } from '$app/forms';
	import { notification } from '$lib/stores';

	let { data }: { data: PageData } = $props();

	const { task, rendered } = data;

	let overdue = $state(false);
	let completed = $state(task.completed);

	let formattedDate = formatDateTime(task.createdAt);
	let isPreloading = $state(false);
</script>

<svelte:head>
	<title>Task From {formattedDate}</title>
</svelte:head>

<div class="flex items-baseline justify-between">
	<div class="flex space-x-4 items-baseline">
		<H1>{task.title}</H1>

		<a href="/download/{task.filePath}" onclick={() => (isPreloading = true)}>
			{#if !isPreloading}
				<Download class="size-6" />
			{:else}
				<Loader2 class="animate-spin" />
			{/if}
		</a>
		<form
			class="flex"
			method="post"
			use:enhance={() => {
				return async ({ result }) => {
					if (result.type === 'success') {
						const message = completed ? 'Marked As Completed' : 'Not Completed';
						notification.set({ message, type: 'success' });
					} else {
						notification.set({
							message: 'Failed to mark as completed',
							type: 'error'
						});
					}
				};
			}}
		>
			<button onclick={() => (completed = !completed)} class="pointer-events-auto" class:overdue>
				{#if completed}
					<CheckSquare class="w-6 h-6" />
				{:else}
					<Square class="w-6 h-6" />
				{/if}
			</button>
			<input type="hidden" name="completed" value={completed} />
			<input type="hidden" name="id" value={task.id} />
		</form>
	</div>
</div>
<div class="markdown ring-2 ring-milk-200 p-4 rounded-lg dark:ring-milk-900">{@html rendered}</div>
