<script lang="ts">
	import { formatDateTime } from '$lib/utils';
	import type { Lesson } from '$lib/types';
	import { lessonsStore, user } from '$lib/stores';
	import SearchBar from './SearchBar.svelte';
	import LessonCardBig from '../cards/LessonCardBig.svelte';

	let search = $state('');
	let filteredLessons: Lesson[] = $state([]);

	$effect(() => {
		const lowercaseQuery = search.toLowerCase();
		filteredLessons = $lessonsStore.filter(
			(lesson: Lesson) =>
				lesson.title.toLowerCase().includes(lowercaseQuery) ||
				lesson.topic.toLowerCase().includes(lowercaseQuery) ||
				lesson.assigneeName.toLowerCase().includes(lowercaseQuery)
		);
	});

	let href = $user.role === 'teacher' ? 't' : 's';
</script>

<div class="flex flex-col items-center justify-center z-50">
	<SearchBar bind:search />

	<div class="grid grid-cols-2 gap-4 absolute top-20 xl:w-[600px]">
		{#if search}
			{#each filteredLessons as lesson}
				<LessonCardBig {lesson} />
			{/each}
		{/if}
	</div>
</div>
