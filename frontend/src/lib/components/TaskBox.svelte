<script lang="ts">
	export let task: App.Task;
	import { CheckSquare, Download, Square } from 'lucide-svelte';
	import { enhance } from '$app/forms';
	import { notification } from '$lib/stores';

	let overdue = false;
	let completed = task.completed;
	const today = new Date();
	today.setHours(0, 0, 0, 0);

	const tomorrow = new Date(today);
	tomorrow.setDate(tomorrow.getDate() + 1);

	const dueDate = new Date(task.due_date);
	dueDate.setHours(0, 0, 0, 0); // Normalize time to midnight for comparison

	overdue = dueDate < today;

	let formattedDate;
	if (dueDate.toDateString() === today.toDateString()) {
		formattedDate = 'today';
	} else if (dueDate.toDateString() === tomorrow.toDateString()) {
		formattedDate = 'tomorrow';
	} else {
		formattedDate = dueDate.toLocaleDateString('en-GB', {
			month: 'short',
			day: 'numeric',
			year: 'numeric'
		});
	}

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

<div class="flex w-full flex-col py-2 shadow border border-sand-900/10 rounded-lg min-h-[150px]" class:completed>
	<div
		id="task-header"
		class="inline-flex py-3 px-5 space-x-1rounded-t-lg text-3xl justify-between"
	>
	<h2 class="">{task.title}</h2>
	<form class="flex items-center space-x-1" method="post" use:enhance action="?/completed">
		<button on:click={() => (completed = !completed)} class="" class:overdue>
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

	<p class="px-5">{@html task.content}</p>

	<div
		id="task-footer"
		class="items-center mt-auto pt-2 px-5 text-sm inline-flex space-x-1 justify-between"
	>
		<p class:overdue class="opacity-60">Due {formattedDate}</p>

		<input type="hidden" name="file" value={task.file} />
		<button on:click={handleDownload}>
			<Download class="w-6 h-6" />
		</button>
	</div>
</div>

<style>
	.overdue {
		@apply text-rust-600 font-bold underline;
	}

	button.overdue {
		@apply text-inherit;
	}

	.completed {
		@apply opacity-50;
	}
</style>
