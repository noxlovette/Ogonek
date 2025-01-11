<script lang="ts">
	import { AudioLines, Book, BookCopy } from 'lucide-svelte';
	import { language, translations } from '$lib/stores';
</script>

<div class="flex flex-col ring-2 ring-milk-200 rounded-lg shadow-md p-2 lg:p-3 xl:p-4 rounded-lg">
	<h2 class="lg:text-lg xl:text-xl">Useful Links</h2>
	<a
		href="https://forvo.com/languages/en/"
		class="lg:px-2 lg:py-4 px-1 py-2 inline-flex"
		target="_blank"
		rel="noopener noreferrer"
	>
		<AudioLines class="size-6 lg:size-7 xl:size-8 mr-2 " />
		Pron
	</a>

	<a
		href="https://dictionary.cambridge.org/"
		class="lg:px-2 lg:py-4 px-1 py-2 inline-flex"
		target="_blank"
		rel="noopener noreferrer"
	>
		<Book class="size-6 lg:size-7 xl:size-8 mr-2" />

		Dictionary
	</a>

	<a
		href="https://www.thesaurus.com/"
		class="lg:px-2 lg:py-4 px-1 py-2 inline-flex"
		target="_blank"
		rel="noopener noreferrer"
	>
		<BookCopy class="size-6 lg:size-7 xl:size-8 mr-2" />
		Thesaurus
	</a>
</div>

<style>
	a:hover {
		@apply transition-all duration-300 text-brick-400 translate-x-1;
	}
</style>
