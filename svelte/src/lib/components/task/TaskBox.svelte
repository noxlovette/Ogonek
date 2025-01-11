<script lang="ts">
	import { CheckSquare, Download, Square } from 'lucide-svelte';
	import { enhance } from '$app/forms';
	import { notification } from '$lib/stores';
	import { formatDate } from '$lib/utils';
	interface Props {
		task: App.Task;
	}

	let { task }: Props = $props();

	let overdue = false;
	let completed = $state(task.completed);
	const formattedDate = formatDate(task.due_date);

	function handleDownload() {
		// Create an anchor element
		const a = document.createElement('a');
		a.href = task.file;
		a.style.display = 'none';

		// Append the anchor to the body (required for Firefox)
		document.body.appendChild(a);

		// Programmatically click the anchor to trigger the download
		a.click();

		// Remove the anchor from the document
		document.body.removeChild(a);

		// Show a notification
		notification.set({ message: 'Downloading file...', type: 'info' });
	}
</script>

<div
	class="flex w-full flex-col py-2 shadow border border-milk-900/10 rounded-lg min-h-[150px]"
	class:completed
>
	<div
		id="task-header"
		class="inline-flex py-3 px-5 space-x-8 text-lg md:text-xl lg:text-2xl xl:text-3xl justify-between items-baseline"
	>
		<h2 class="flex">{task.title}</h2>
		<form class="flex" method="post" use:enhance action="?/completed">
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

	<p class="px-5 text-sm lg:text-base">{@html task.content}</p>

	<div
		id="task-footer"
		class="items-center mt-auto pt-2 px-5 text-sm inline-flex space-x-1 justify-between"
	>
		<p class:overdue class="opacity-60">Due {formattedDate}</p>

		{#if task.file}
			<button onclick={handleDownload}>
				<Download class="w-6 h-6" />
			</button>
		{/if}
	</div>
</div>

<style>
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
