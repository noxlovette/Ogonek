<script lang="ts">
	import { getContext } from 'svelte';
	import TaskBox from './TaskBox.svelte';
	import { Bird } from 'lucide-svelte';

	let tasks: App.Task[] = getContext('tasks');


	let filtered = tasks.filter(task => task.completed === false);
	let completed = tasks.filter(task => task.completed === true);

	let completedVisible = false;

</script>


{#if filtered.length > 0}
<h1 class="text-4xl">
    My Tasks
</h1>
<div class="grid grid-cols-1 gap-4 md:grid-cols-2 my-6 w-full">
		{#each filtered as task}
			<TaskBox {task} />
		{/each}
	</div>
	
{:else}
<Bird class="size-32 mx-auto text-sand-900" />
	<p>No tasks</p>
	
{/if}
<button on:click={()=> completedVisible = !completedVisible} class="text-sm font-sans p-1 hover:text-sand-800">
		
	{#if completedVisible}
	Hide
	{:else}
	View 
	{/if}
	Completed</button>

{#if completedVisible}
<div class='grid grid-cols-2 my-4 gap-4'>
    {#each completed as task}
		<TaskBox {task} />
    {/each}
</div>
{/if}