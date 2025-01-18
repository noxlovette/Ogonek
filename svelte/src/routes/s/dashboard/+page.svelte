<script lang="ts">
	import Header from '$lib/components/typography/H1.svelte';
	import {lessonStore} from '$lib/stores';
	import { getGreeting } from '$lib/utils';
</script>

<svelte:head>
	<title>Dashboard</title>
</svelte:head>

<div class="w-full flex flex-col">
	<Header>Recent Activity</Header>
	<div class="grid gap-4 grid-cols-1 md:grid-cols-2 w-full my-4">
		<div class="flex flex-col space-y-2">
			<h2 class="text-2xl">New Tasks</h2>
		</div>
		<div class="flex flex-col space-y-2">
			<h2 class="text-2xl">New Lessons</h2>
		</div>
	</div>
</div>
