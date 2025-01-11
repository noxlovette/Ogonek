<script lang="ts">
	import Header from '$lib/components/fonts/Header.svelte';
	import LessonCardBig from '$lib/components/lesson/LessonCardBig.svelte';
	import { getContext } from 'svelte';
	import { language, translations } from '$lib/stores';

	const lessons: App.Lesson[] = getContext('lessons');
</script>

<Header>
	Lessons
</Header>
<div class="grid grid-cols-2 gap-4">
	{#each lessons as lesson}
		<LessonCardBig {lesson} />
	{/each}
</div>

<a href="/u/lessons/bookmarked" class="text-sm font-sans p-1 hover:text-brick-800">
	View Bookmarked
</a>
