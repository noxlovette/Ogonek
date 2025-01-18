<script lang="ts">
	import { formatDateTime } from '$lib/utils';
	import type { Lesson } from '$lib/types';
	import { lessonsStore, user } from '$lib/stores';
	import SearchBar from './SearchBar.svelte';
	import LessonCardBig from '$lib/components/cards/LessonCard.svelte';

	let query = $state('');
	let filteredLessons: Lesson[] = $state([]);

	$effect(() => {
		const lowercaseQuery = query.toLowerCase();
		filteredLessons = query
			? $lessonsStore.filter((lesson) =>
					[lesson.title, lesson.topic, lesson.assigneeName].some((field) =>
						field.toLowerCase().includes(lowercaseQuery)
					)
				)
			: [];
	});

	let href = $user.role === 'teacher' ? 't' : 's';
</script>

<div class="flex flex-col flex-1 items-center w-full justify-center z-50">
	<SearchBar bind:query />

	<div class="max-h-96 overflow-y-auto">
		{#if query}
			{#each filteredLessons as lesson}
				<LessonCardBig {lesson} />
			{/each}
		{/if}
	</div>
</div>
