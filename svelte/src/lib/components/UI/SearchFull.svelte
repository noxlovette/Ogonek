<script lang="ts">
	import { getContext } from 'svelte';
	import { formatDateTime } from '$lib/utils';
	import { Search, X } from 'lucide-svelte';

	import type { Lesson } from '$lib/types';
	import SearchBar from './SearchBar.svelte';

	const lessons: Lesson[] = getContext('lessons');

	let search = $state('');
	let filteredLessons: Lesson[] = $state([]);

	function filterAndHighlight(searchTerm: string, items: Lesson[]) {
		const term = searchTerm.toLowerCase().trim();
		if (!term) return items;

		function highlight(content: string): string {
			try {
				const startIndex = content.toLowerCase().indexOf(term);
				if (startIndex === -1) return content;

				// Grab chunk around the match
				const start = Math.max(0, startIndex - 30);
				const end = Math.min(content.length, startIndex + term.length + 30);
				let result = content.slice(start, end);

				// Add ellipsis if we trimmed content
				if (start > 0) result = '...' + result;
				if (end < content.length) result += '...';

				// Highlight the term itself
				return result.replace(
					new RegExp(term, 'gi'),
					(match) => `<mark class="bg-brick-900 text-brick-100">${match}</mark>`
				);
			} catch {
				// If anything goes wrong, return original
				return content;
			}
		}

		return items
			.filter(
				(item) =>
					item.topic.toLowerCase().includes(term) || item.markdown.toLowerCase().includes(term)
			)
			.map((item) => ({
				...item,
				highlighted: highlight(item.markdown)
			}));
	}

	$effect(() => {
		filterAndHighlight(search, lessons);
	});
</script>

<div class="flex flex-col items-center justify-center">
	<SearchBar bind:search />

	<div class="grid grid-cols-2 gap-4 absolute top-20 xl:w-[600px]">
		{#if search}
			{#each filteredLessons as lesson}
				<a
					href="/u/lessons/l/{lesson.id}"
					class="flex flex-col hover:bg-brick-900/20 hover:text-brick-100 transition-colors shadow border border-milk-900/10 rounded-lg max-h-[150px] overflow-clip py-2"
				>
					<div class="flex flex-col py-3 px-5">
						<h3 class="text-lg md:text-xl lg:text-2xl xl:text-3xl">
							{formatDateTime(lesson.manualDate || lesson.createdAt)}
						</h3>

						<p class="opacity-80 highlighted-content text-sm md:text-base">
							{@html lesson.highlighted}
						</p>
					</div>
				</a>
			{/each}
		{/if}
	</div>
</div>
