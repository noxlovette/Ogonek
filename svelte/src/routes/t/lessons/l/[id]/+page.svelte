<script lang="ts">
	import { formatDateTime } from '$lib/utils';
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();

	const { lesson, rendered } = data;

	let formattedDate = formatDateTime(lesson.createdAt);
</script>

<svelte:head>
	<title>Lesson from {formattedDate}</title>
</svelte:head>

<a href="/t/lessons/l/{lesson.id}/edit" class="bg-red-200">Edit</a>

<article class="">
	<div id="header" class="flex flex-col md:flex-row items-baseline mb-4">
		<h1 class="text-4xl font-bold">{lesson.title}</h1>
		<h2><span class="text-sm opacity-60 font-bold mr-1">Topic:</span> {lesson.topic}</h2>
	</div>
	<div class="markdown">{@html rendered}</div>
</article>
