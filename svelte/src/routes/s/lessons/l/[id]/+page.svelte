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
	<title>Lesson From {formattedDate}</title>
</svelte:head>

{#if lesson}
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
{:else}
	This lesson has been deleted
{/if}
