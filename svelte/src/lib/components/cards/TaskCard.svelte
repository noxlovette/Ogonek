<script lang="ts">
	import { CheckSquare, Download, Square } from 'lucide-svelte';
	import { enhance } from '$app/forms';
	import { notification } from '$lib/stores';
	import { formatDateTime } from '$lib/utils';
	import type { Task } from '$lib/types';
	import Card from './Card.svelte';
	interface Props {
		task: Task;
	}

	let { task }: Props = $props();

	let overdue = false;
	let completed = $state(task.completed);
	const formattedDate = formatDateTime(task.dueDate);

	function handleDownload() {
		const a = document.createElement('a');
		a.href = task.filePath;
		a.style.display = 'none';
		document.body.appendChild(a);
		a.click();
		document.body.removeChild(a);
		notification.set({ message: 'Downloading file...', type: 'info' });
	}
</script>

<Card>
	<div
		id="task-header"
		class="inline-flex space-x-8 text-lg md:text-xl lg:text-2xl xl:text-3xl justify-between items-baseline"
	>
		<h2 class="flex">{task.title}</h2>
		<form
			class="flex"
			method="post"
			use:enhance={() => {
				return async ({ result }) => {
					if (result.type === 'success') {
						notification.set({ message: 'Marked as completed', type: 'success' });
					} else {
						notification.set({
							message: 'Failed to mark as completed',
							type: 'error'
						});
					}
				};
			}}
			action="?/completed"
		>
			<button onclick={() => (completed = !completed)} class="" class:overdue>
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

	<p class="text-sm lg:text-base">{@html task.markdown}</p>

	<div id="task-footer" class="items-center mt-auto text-sm inline-flex space-x-1 justify-between">
		<p class:overdue class="opacity-60">Due {formattedDate}</p>

		{#if task.filePath}
			<button onclick={handleDownload}>
				<Download class="w-6 h-6" />
			</button>
		{/if}
	</div>
</Card>

<style lang="postcss">
	.overdue {
		@apply text-brick-600 font-bold underline;
	}

	button.overdue {
		@apply text-inherit;
	}

	.completed {
		@apply opacity-50;
	}
</style>
