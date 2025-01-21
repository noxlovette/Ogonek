<script lang="ts">
	import type { PageData } from './$types';
	import { H2, LessonCard, H1 } from '$lib/components';

	let { data }: { data: PageData } = $props();
	const { lessons } = data;

	$effect(() => {
		lessons.sort((a, b) => new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime());
	});
</script>

<svelte:head>
	<title>Lessons</title>
</svelte:head>

<H1>Lessons</H1>
<section class="space-y-4">
	<H2>Recent Lessons</H2>
	<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
		{#each lessons as lesson}
			<LessonCard {lesson} />
		{/each}
	</div>
</section>
