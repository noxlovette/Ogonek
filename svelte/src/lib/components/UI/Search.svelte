<script lang="ts">
	import { run } from 'svelte/legacy';

	import { getContext } from 'svelte';
	import { formatDate } from '$lib/utils';
	import { Search, X } from 'lucide-svelte';
	import { tweened } from 'svelte/motion';
	import { cubicOut, quadInOut, quadIn } from 'svelte/easing';
	import { language, translations } from '$lib/stores';

	const lessons = getContext('lessons');

	interface Props {
		// const tasks = getContext('tasks');
		hidden?: boolean;
	}

	let { hidden = $bindable(false) }: Props = $props();

	const topPosition = tweened(0, {
		duration: 20,
		easing: quadIn
	});
	function moveToTop() {
		hidden = true;
		// topPosition.set(-300); // Adjust this value as per your layout
	}

	function moveBack() {
		hidden = false;

		// topPosition.set(0);
	}
	let search = $state('');
	let filteredLessons = $state([]);
	let filteredTasks = [];

	function filterItems() {
		const searchTerm = search.toLowerCase();

		const highlightMatches = (content: string): string => {
			if (!searchTerm) return content;
			const regex = new RegExp(searchTerm, 'gi');
			const match = content.match(regex);
			if (!match) return content;

			const startIndex = content.toLowerCase().indexOf(match[0].toLowerCase());
			const endIndex = startIndex + match[0].length;

			// Here, you're slicing 50 characters before and after the match, adjust as needed
			const start = Math.max(0, startIndex - 30);
			const end = Math.min(content.length, endIndex + 30);

			let slicedContent = content.slice(start, end);
			if (start > 0) slicedContent = '... ' + slicedContent;
			if (end < content.length) slicedContent += ' ...';

			return slicedContent.replace(
				regex,
				(match) => `<mark class="bg-forest-800 text-sand-100">${match}</mark>`
			);
		};

		// Filter and highlight lessons
		filteredLessons = lessons
			.map((lesson) => ({
				...lesson,
				highlightedContent: highlightMatches(lesson.content)
			}))
			.filter(
				(lesson) =>
					lesson.topic.toLowerCase().includes(searchTerm) || // Assuming 'topic' instead of 'title'
					lesson.content.toLowerCase().includes(searchTerm)
			);
	}

	run(() => {
		search, filterItems();
	});
</script>

<div class="flex flex-col w-full items-center justify-center">
	<div class="inline-flex group caret-sand-900 focus:shadow-lg lg:text-lg xl:text-xl relative">
		<div
			class="rounded-l-full border-sand-400 border-r-0 border-2 bg-sand-50 px-4 py-2 my-4 group-focus-within:border-forest-700 group-focus-within:bg-forest-50 transition-colors group-focus-within:text-forest-700/70"
		>
			<Search
				class=" size-6 lg:size-7 xl:size-8 text-sand-400 group-focus-within:text-forest-700/70"
			/>
		</div>
		<input
			type="text"
			placeholder="Search Lessons"
			bind:value={search}
			class="border-sand-400 pl-0 bg-sand-50 border-2 border-l-0 focus:outline-none focus:border-forest-700 focus:bg-forest-50 placeholder:text-sand-900/70 focus:placeholder:text-forest-700/70 transition-colors rounded-r-full px-4 py-2 my-4"
			onfocus={moveToTop}
			onblur={moveBack}
		/>
	</div>

	<div class="grid grid-cols-2 gap-4 absolute top-20 xl:w-[600px]">
		{#if search}
			{#each filteredLessons as lesson}
				<a
					href="/u/lessons/l/{lesson.id}"
					class="flex flex-col hover:bg-sand-900/20 hover:text-sand-100 transition-colors shadow border border-sand-900/10 rounded-lg max-h-[150px] overflow-clip py-2"
				>
					<div class="flex flex-col py-3 px-5">
						<h3 class="text-lg md:text-xl lg:text-2xl xl:text-3xl">
							{formatDate(lesson.manual_date || lesson.created_at)}
						</h3>

						<p class="opacity-80 highlighted-content text-sm md:text-base">
							{@html lesson.highlightedContent}
						</p>
					</div>
				</a>
			{/each}
		{/if}
	</div>
</div>
