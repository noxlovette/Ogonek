<script lang="ts">
	import { H1, H2 } from '$lib/components';
	import ButtonRaw from '$lib/components/UI/buttons/ButtonRaw.svelte';
	import { lessonStore, user } from '$lib/stores';
	import { formatDateTime } from '$lib/utils';
	import type { PageData } from './$types';
	import { goto } from '$app/navigation';

	let { data }: { data: PageData } = $props();

	const { task, rendered } = data;

	let formattedDate = formatDateTime(task.createdAt);

	console.log(task);
</script>

<svelte:head>
	<title>Task from {formattedDate}</title>
</svelte:head>

<div class="flex items-center justify-between">
	<div class="flex space-x-4">
		<H1>{task.title}</H1>
		<a
			href="/t/tasks/t/{task.id}/edit"
			class="px-4 py-2 bg-brick-600 text-brick-50 rounded-lg hover:bg-brick-700 focus:outline-none focus:ring-2 focus:ring-brick-500 focus:ring-offset-2 disabled:opacity-50 transition-colors"
			>Edit</a
		>
	</div>
	<div class="text-right">
		<p class="block font-medium text-milk-700">Student</p>
		<h3 class="">
			{#if task.assigneeName === $user.username}
				Not Assigned
			{:else}
				{task.assigneeName}
			{/if}
		</h3>
	</div>
</div>
<H2>Content</H2>
<div class="markdown ring-2 ring-milk-200 p-4 rounded-lg">{@html rendered}</div>

{#if task.filePath}
	<div>
		<ButtonRaw
			onclick={() => goto(`/download/${encodeURIComponent(task.filePath)}`)}
			buttonName="Download"
		/>
	</div>
{/if}
