<script lang="ts">
	import LessonCard from './LessonCard.svelte';
	import { language, translations } from '$lib/stores';
	import type { Lesson } from '$lib/types';

	import { getContext } from 'svelte';

	let lessons: Lesson[] = $state(getContext('lessons') || []);

	if (lessons.length > 3) {
		lessons = lessons.slice(0, 3);
	}
</script>

<div class="p-2 lg:p-3 xl:p-4 flex flex-col my-4 ring-2 ring-brick-200 rounded-lg shadow-md">
	<h2 class=" lg:text-lg xl:text-xl">Recent</h2>
	<ul class="">
		{#if lessons.length === 0}
			<p>No lessons found</p>
		{:else}
			{#each lessons as lesson}
				<LessonCard {lesson} />
			{/each}
		{/if}
	</ul>
	<a href="/u/lessons" class="text-brick-400 text-xs lg:text-sm font-sans p-1"> View All </a>
</div>

<style>
	a:hover {
		@apply transition-all duration-300 text-brick-400 translate-x-1;
	}
</style>
