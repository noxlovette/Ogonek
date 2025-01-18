<script lang="ts">
	import TaskBox from '$lib/components/cards/TaskCard.svelte';
	import { taskStore } from '$lib/stores';

	import { getContext } from 'svelte';

	import { Bird } from 'lucide-svelte';
	import Header from '$lib/components/typography/H1.svelte';

	let filtered = tasks.filter((task) => task.completed === false);
	let completed = tasks.filter((task) => task.completed === true);

	let completedVisible = $state(false);
</script>

<svelte:head>
	<title>Tasks</title>
</svelte:head>
