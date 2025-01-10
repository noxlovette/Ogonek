<script lang="ts">
	import UsefulLInks from './UsefulLInks.svelte';
	import Word from './Word.svelte';
	import Lessons from '$lib/components/lesson/Lessons.svelte';
	import { Send } from 'lucide-svelte';
	import { language, translations } from '$lib/stores';
</script>

<div
	class="hidden select-none flex-col h-full min-h-screen text-sand-100 font-serif lg:text-xl xl:text-2xl w-1/4 px-4 pb-4 md:flex"
>
	<a
		href="https://t.me/noxlovette"
		target="_blank"
		class="px-1 py-2 my-2 lg:px-2 lg:py-4 inline-flex"
	>
		<Send class="size-6 lg:size-7 xl:size-8 mr-2 " />

		{$translations.text_me[$language]}
	</a>
	<Word></Word>
	<Lessons></Lessons>
	<UsefulLInks></UsefulLInks>
	<div class="mt-auto xl:text-base text-xs">
		<p>© 2024 Danila Volkov</p>
		<p class="italic">For my students <span class="not-italic">💛</span></p>
	</div>
</div>

<style>
	a:hover {
		@apply bg-sand-900/20 transition-colors duration-300 rounded-lg text-sand-100;
	}
</style>
