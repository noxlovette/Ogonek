<script lang="ts">
	import { H1 } from '$lib/components';
	import { user } from '$lib/stores';
	import { formatDateTime } from '$lib/utils';
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();
	const { lesson, rendered } = data;

	let formattedDate = formatDateTime(lesson.createdAt);
</script>

<svelte:head>
	<title>Lesson from {formattedDate}</title>
</svelte:head>

{#if lesson}
	<div class="flex items-baseline space-x-4">
		<H1>{lesson.title}</H1>
		<a
			href="/t/lessons/l/{lesson.id}/edit"
			class="px-4 py-2 bg-brick-600 text-brick-50 rounded-lg hover:bg-brick-700 focus:outline-none focus:ring-2 focus:ring-brick-500 focus:ring-offset-2 disabled:opacity-50 transition-colors"
			>Edit</a
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
	<div class="markdown ring-2 ring-milk-200 p-4 rounded-lg">{@html rendered}</div>
{:else}
	This lesson has been deleted
{/if}
