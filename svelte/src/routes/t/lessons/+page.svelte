<script lang="ts">
	import type { PageData } from './$types';
	import type { Lesson } from '$lib/types';
	import { getContext } from 'svelte';
	import LessonCardBig from '$lib/components/cards/LessonCardBig.svelte';
	let { data }: { data: PageData } = $props();

	const lessons: Lesson[] = getContext('lessons');
</script>

<div class="grid grid-cols-2 gap-4">
	{#each lessons as lesson}
		<LessonCardBig {lesson} />
	{/each}
</div>
