<script lang="ts">
	import { getContext } from 'svelte';
	import TaskBox from './TaskBox.svelte';

	let tasks: App.Task[] = getContext('tasks');

	console.log(tasks);
</script>

<h1 class="text-2xl font-bold">Tasks</h1>
{#if tasks.length > 0}
	<div class="grid grid-cols-1 gap-4 md:grid-cols-2 my-6">
		{#each tasks as task}
			<TaskBox {task} />
		{/each}
	</div>
{:else}
	<p>No tasks found</p>
{/if}
