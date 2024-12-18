<script lang="ts">
	import TaskBox from '$lib/components/TaskBox.svelte';

	import type { PageServerData } from './$types';
	export let data:PageServerData;

	import { getContext } from 'svelte';

	import { Bird } from 'lucide-svelte';
	import Header from '$lib/components/fonts/Header.svelte';

	let tasks: App.Task[] = getContext('tasks');

	let filtered = tasks.filter(task => task.completed === false);
	let completed = tasks.filter(task => task.completed === true);

	let completedVisible = false;

</script>


{#if filtered.length > 0}

<Header>
	My Tasks
</Header>
<div class="grid grid-cols-1 gap-4 md:grid-cols-2 my-4 w-full">
		{#each filtered as task}
			<TaskBox {task} />
		{/each}
</div>
	
{:else}
<Bird class="size-32 mx-auto text-sand-900" />
	<p>No tasks</p>
	
{/if}
<button on:click={()=> completedVisible = !completedVisible} class="text-sm font-sans text-left hover:text-sand-800">
		
	{#if completedVisible}
	Hide
	{:else}
	View 
	{/if}
	Completed

</button>

{#if completedVisible}
<div class='grid grid-cols-2 my-4 gap-4'>
    {#each completed as task}
		<TaskBox {task} />
    {/each}
</div>
{/if}
