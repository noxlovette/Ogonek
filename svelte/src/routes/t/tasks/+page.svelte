<script lang="ts">
	import type { PageData } from './$types';
	import { getContext } from 'svelte';
	import type { Task } from '$lib/types';
	import TaskCard from '$lib/components/cards/TaskCard.svelte';
	import Header from '$lib/components/typography/Header.svelte';
	import NewCard from '$lib/components/cards/NewCard.svelte';

	let { data }: { data: PageData } = $props();

	const tasks: Task[] = getContext('tasks');
</script>

<Header>Tasks</Header>
<div class="grid grid-cols-2 gap-4">
	<NewCard href="?/new" />
	{#each tasks as task}
		<TaskCard {task} />
	{/each}
</div>
