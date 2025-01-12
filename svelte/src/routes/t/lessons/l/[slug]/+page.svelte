<script lang="ts">
	import type { PageServerData } from './$types';

	let { data }: { data: PageServerData } = $props();

	const { lesson } = data;

	let date = $derived(new Date(lesson.manualDate || lesson.createdAt));

	let formattedDate = $derived(
		date.toLocaleDateString('en-GB', {
			month: 'short',
			day: 'numeric',
			year: 'numeric'
		})
	);
</script>

<svelte:head>
	<title>Lesson from {formattedDate}</title>
</svelte:head>

<article class="flex flex-col text-lg size-full">
	<div id="header" class="flex flex-col md:flex-row items-start justify-between">
		<h1 class="text-4xl font-bold">{formattedDate}</h1>
		<div class="flex flex-col md:border-2 md:px-2 md:py-1 py-4 rounded-lg border-milk-900/60">
			<h2><span class="text-sm opacity-60 font-bold mr-1">Topic:</span> {lesson.topic}</h2>
			<h3>
				<span class="text-sm opacity-60 font-bold mr-1">Category:</span>{lesson.category}
			</h3>
		</div>
	</div>
	<div class="markdown">{@html lesson.markdown}</div>
</article>
