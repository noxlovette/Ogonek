<script lang="ts">
	import type { PageData } from './$types';
	import type { Lesson } from '$lib/types';
	import { getContext } from 'svelte';
	let { data }: { data: PageData } = $props();

	const lessons: Lesson[] = getContext('lessons');
</script>

{#each lessons as lesson}
	<div>{lesson.title}</div>
{/each}
