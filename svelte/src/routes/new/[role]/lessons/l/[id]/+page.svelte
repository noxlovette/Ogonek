<script lang="ts">
	import { H1 } from '$lib/components';
	import { user } from '$lib/stores';
	import { formatDateTime } from '$lib/utils';
	import type { PageData } from './$types';
	import { goto } from '$app/navigation';
    import { page } from '$app/state';

	let { data }: { data: PageData } = $props();

	const { lesson, rendered } = data;

	let formattedDate = formatDateTime(lesson.createdAt);
</script>

{#if page.params.role === "t"}
	<div class="flex items-baseline space-x-4">
		<H1>{lesson.title}</H1>
		<button
			onclick={() => goto(`/t/lessons/l/${lesson.id}/edit`, { replaceState: true })}
			class="px-4 py-2 bg-cacao-600 text-cacao-50 rounded-lg hover:bg-cacao-700 focus:outline-none focus:ring-2 focus:ring-cacao-500 focus:ring-offset-2 disabled:opacity-50 transition-colors"
			>Edit</button
		>
	</div>
	<div class="flex space-x-4">
		<div class="space-y-2">
			<p class="block font-medium text-milk-700">Topic</p>
			<h3 class="min-w-48">
				{lesson.topic}
			</h3>
		</div>
		<div class="space-y-2">
			<p class="block font-medium text-milk-700">Student</p>
			<h3 class="min-w-48">
				{#if lesson.assigneeName === $user.username}
					Not Assigned
				{:else}
					{lesson.assigneeName}
				{/if}
			</h3>
		</div>
	</div>
	<h3 class="text-2xl font-bold">Content</h3>
	<div class="markdown ring-2 ring-milk-200 dark:ring-milk-900 p-4 rounded-lg">
		{@html rendered}
	</div>
	{:else}
	<div class="flex items-baseline space-x-4">
		<H1>{lesson.title}</H1>
	</div>
	<div class="">
		<p class="block font-medium text-milk-700">Topic</p>
		<h3 class="min-w-48">
			{lesson.topic}
		</h3>
	</div>
	<div class="markdown ring-2 dark:ring-milk-900 ring-milk-200 p-4 rounded-lg">
		{@html rendered}
	</div>
	{/if}

	<svelte:head>
	<title>Lesson from {formattedDate}</title>
</svelte:head>