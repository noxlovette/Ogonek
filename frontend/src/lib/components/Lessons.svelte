<script lang="ts">
	import LessonCard from './LessonCard.svelte';

	import { getContext } from 'svelte';

	const lessons: App.Lesson[] = getContext('lessons');
</script>

<div class="p-4 border-2 rounded-lg border-sand-900/20">
	<h2 class="text-xl">Recent Lessons</h2>
	<ul class="">
		{#each lessons as lesson}
			<LessonCard {lesson} />
		{/each}
	</ul>
	<a href="/u/lessons" class="text-sand-400 text-sm font-sans p-1">View All Lessons</a>
</div>

<style>
	a:hover {
		@apply bg-sand-900/20 transition-colors duration-300 rounded-lg text-sand-100;
	}
</style>
