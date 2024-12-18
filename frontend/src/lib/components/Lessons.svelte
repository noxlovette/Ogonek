<script lang="ts">
	import LessonCard from './LessonCard.svelte';

	import { getContext } from 'svelte';

	let lessons: App.Lesson[] = getContext('lessons') || [];

	if (lessons.length > 3) {
		lessons = lessons.slice(0, 3);
	}

</script>

<div class="p-2 lg:p-3 xl:p-4 border-2 flex flex-col my-4 rounded-lg border-sand-900/20">
	<h2 class=" lg:text-lg xl:text-xl">Recent Lessons</h2>
	<ul class="">
		{#if lessons.length === 0}
			<p>No lessons found</p>
		{:else}
			{#each lessons as lesson}
				<LessonCard {lesson} />
			{/each}
		{/if}
	</ul>
	<a href="/u/lessons" class="text-sand-400 text-xs lg:text-sm font-sans p-1">View All Lessons</a>
</div>

<style>
	a:hover {
		@apply bg-sand-900/20 transition-colors duration-300 rounded-lg text-sand-100;
	}
</style>
