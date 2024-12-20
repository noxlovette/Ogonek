<script lang="ts">
	import { AudioLines, Book, BookCopy } from 'lucide-svelte';
	import { language, translations } from '$lib/stores';
</script>

<div class="flex flex-col border-2 border-sand-900/20 p-2 lg:p-3 xl:p-4 rounded-lg">
	<h2 class="lg:text-lg xl:text-xl">
		{$translations.useful_links[$language]}
	</h2>
	<a
		href="https://forvo.com/languages/{$language}/"
		class="lg:px-2 lg:py-4 px-1 py-2 inline-flex"
		target="_blank"
		rel="noopener noreferrer"
	>
		<AudioLines class="size-6 lg:size-7 xl:size-8 mr-2 " />
		{$translations.pronunciation[$language]}
	</a>

	{#if $language === 'de'}
		<a
			href="https://www.dwds.de/"
			class="lg:px-2 lg:py-4 px-1 py-2 inline-flex"
			target="_blank"
			rel="noopener noreferrer"
		>
			<Book class="size-6 lg:size-7 xl:size-8 mr-2" />

			{$translations.dictionary[$language]}
		</a>
		<a
			href="https://www.openthesaurus.de/"
			class="lg:px-2 lg:py-4 px-1 py-2 inline-flex"
			target="_blank"
			rel="noopener noreferrer"
		>
			<BookCopy class="size-6 lg:size-7 xl:size-8 mr-2" />
			{$translations.thesaurus[$language]}
		</a>
	{:else}
		<a
			href="https://dictionary.cambridge.org/"
			class="lg:px-2 lg:py-4 px-1 py-2 inline-flex"
			target="_blank"
			rel="noopener noreferrer"
		>
			<Book class="size-6 lg:size-7 xl:size-8 mr-2" />

			{$translations.dictionary[$language]}
		</a>

		<a
			href="https://www.thesaurus.com/"
			class="lg:px-2 lg:py-4 px-1 py-2 inline-flex"
			target="_blank"
			rel="noopener noreferrer"
		>
			<BookCopy class="size-6 lg:size-7 xl:size-8 mr-2" />
			{$translations.thesaurus[$language]}
		</a>
	{/if}
</div>

<style>
	a:hover {
		@apply bg-sand-900/20 transition-colors duration-300 rounded-lg text-sand-100;
	}
</style>
