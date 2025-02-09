<script lang="ts">
	/**
	 * @component
	 * @description Searches for tasks, lessons, and lives in the header
	 */

	import type { Lesson, Task } from '$lib/types';
	import { lessonStore, taskStore } from '$lib/stores';
	import SearchBar from './SearchBar.svelte';
	import LessonCard from '$lib/components/cards/LessonCard.svelte';
	import TaskCard from '$lib/components/cards/TaskCard.svelte';

	let query = $state('');
	let filteredLessons: Lesson[] = $state([]);
	let filteredTasks: Task[] = $state([]);

	$effect(() => {
		const lowercaseQuery = query.toLowerCase();

		filteredLessons = $lessonStore.filter((lesson) =>
			[lesson.title, lesson.topic, lesson.assigneeName, lesson.markdown].some((field) =>
				field.toLowerCase().includes(lowercaseQuery)
			)
		);

		filteredTasks = $taskStore.filter((task) =>
			[task.title, task.markdown].some((field) => field.toLowerCase().includes(lowercaseQuery))
		);
	});
</script>

<div class="z-40">
	<SearchBar bind:query />

	<div
		class="absolute top-20 left-1/2 w-11/12 -translate-x-1/2 md:w-full max-w-2xl dark:bg-milk-900 bg-cacao-50 rounded-lg shadow-xl"
	>
		{#if query}
			<div class="max-h-[32rem] overflow-y-auto divide-y divide-milk-200">
				{#if filteredLessons.length}
					<section class="p-4">
						<h2 class="text-sm font-medium text-milk-500 mb-3">Lessons</h2>
						<div class="space-y-2">
							{#each filteredLessons as lesson}
								<LessonCard {lesson} />
							{/each}
						</div>
					</section>
				{/if}

				{#if filteredTasks.length}
					<section class="p-4">
						<h2 class="text-sm font-medium text-milk-500 mb-3">Tasks</h2>
						<div class="space-y-2">
							{#each filteredTasks as task}
								<TaskCard interactive={false} {task} />
							{/each}
						</div>
					</section>
				{/if}

				{#if !filteredLessons.length && !filteredTasks.length}
					<div class="p-8 text-center text-milk-500">No results found</div>
				{/if}
			</div>
		{/if}
	</div>
</div>
