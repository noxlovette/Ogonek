<script lang="ts">
	export let task: App.Task;
	import { CheckSquare, Download, Square } from 'lucide-svelte';
	import { enhance } from '$app/forms';
	import { Result } from 'postcss';

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
    a.href = task.file; // Specify the file path
    a.download = 'roses-center.svg'; // Specify the filename for download
    a.style.display = 'none';
    
    // Append the anchor to the body (required for Firefox)
    document.body.appendChild(a);
    
    // Programmatically click the anchor to trigger the download
    a.click();
    
    // Remove the anchor from the document
    document.body.removeChild(a);
  }
</script>

<div class="flex flex-col shadow rounded-lg" class:completed>
	<div
		id="task-header"
		class="inline-flex space-x-1 p-2 bg-forest-700/90 text-forest-50 rounded-t-lg text-xl"
		class:overdue
	>
		<form class="flex items-center space-x-1" method="post" use:enhance action="?/completed">
			<button on:click={() => (completed = !completed)} class="text-forest-500" class:overdue>
				{#if completed}
					<CheckSquare class="w-6 h-6" />
				{:else}
					<Square class="w-6 h-6" />
				{/if}
			</button>
			<input type="hidden" name="completed" value={completed} />
			<input type="hidden" name="id" value={task.id} />
		</form>
		<h2 class="font-bold">{task.title}</h2>
	</div>
	<p class="opacity-80 p-2">{@html task.content}</p>
	<div
		id="task-footer"
		class="items-center mt-auto text-sm inline-flex space-x-1 p-2 opacity-60 justify-between"
	>
		<p class="">{formattedDate}</p>
		
			<input type="hidden" name="file" value={task.file} />
			<button on:click={handleDownload}>
				<Download class="w-6 h-6" />
			</button>

	</div>
</div>

<style>
	.overdue {
		@apply bg-rust-400 text-rust-50;
	}

	button.overdue {
		@apply text-rust-200;
	}

	.completed {
		@apply opacity-50;
	}
</style>
