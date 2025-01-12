<script lang="ts">
	import type { PageData } from './$types';
	import { getContext } from 'svelte';
	import type { Task } from '$lib/types';

	let { data }: { data: PageData } = $props();

	const tasks: Task[] = getContext('tasks');
</script>

{#each tasks as task}
	<div>{task.markdown}</div>
{/each}
