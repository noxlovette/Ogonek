<script lang="ts">
	import { getContext } from 'svelte';
	import { formatDate } from '$lib/utils';
	import { Search } from 'lucide-svelte';
	import { tweened } from 'svelte/motion';
	import { cubicOut, quadInOut, quadIn } from 'svelte/easing';

	const lessons = getContext('lessons');
	const tasks = getContext('tasks');
	export let hidden = false;

	const topPosition = tweened(0, {
		duration: 20,
		easing: quadIn
	});
	function moveToTop() {
		hidden = true;
		topPosition.set(-300); // Adjust this value as per your layout
	}

	function moveBack() {
		hidden = false;
		topPosition.set(0);
	}
	let search = '';
	let filteredLessons = [];
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

		// Filter and highlight tasks (if needed)
		filteredTasks = tasks
			.map((task) => ({
				...task,
				highlightedContent: highlightMatches(task.content)
			}))
			.filter(
				(task) =>
					task.title.toLowerCase().includes(searchTerm) || // Assuming tasks have 'title'
					task.content.toLowerCase().includes(searchTerm)
			);
	}

	$: search, filterItems();
</script>

<div
	class="flex flex-col items-center justify-center"
	style="transform: translateY({$topPosition}px); transition: transform 0.3s;"
>
	<div class="inline-flex group caret-sand-900 focus:shadow-lg text-xl relative">
		<div
			class="rounded-l-full border-sand-900/60 border-r-0 border-2 bg-sand-900/60 px-4 py-2 my-4 group-focus-within:border-sand-900 group-focus-within:bg-sand-100 transition-colors group-focus-within:text-sand-100"
		>
			<Search class="w-8 h-8 text-sand-100 group-focus-within:text-sand-900/70" />
		</div>
		<input
			type="text"
			placeholder="Search lessons"
			bind:value={search}
			class="border-sand-900/60 pl-0 bg-sand-900/60 text-sand-900 border-2 border-l-0 focus:outline-none focus:border-sand-900 focus:bg-sand-100 placeholder:text-sand-100 focus:placeholder:text-sand-900/70 transition-colors rounded-r-full px-4 py-2 my-4"
			on:focus={moveToTop}
			on:blur={moveBack}
		/>
	</div>

	<div class="grid grid-cols-2 gap-4 absolute top-20 w-[600px]">
		{#if search}
			{#each filteredLessons as lesson}
				<a
					href="/u/lessons/l/{lesson.id}"
					class="flex flex-col hover:bg-sand-900/60 hover:text-sand-100 transition-colors shadow border border-sand-900/10 rounded-lg max-h-[150px] overflow-clip py-2"
				>
					<div class="flex flex-col py-3 px-5">
						<h3 class="text-3xl">
							{formatDate(lesson.manual_date || lesson.created_at)}
						</h3>

						<p class="opacity-80 highlighted-content">
							{@html lesson.highlightedContent}
						</p>
					</div>
				</a>
			{/each}
		{/if}
	</div>
</div>
