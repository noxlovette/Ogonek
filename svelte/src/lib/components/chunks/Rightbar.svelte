<script lang="ts">
	import UsefulLInks from './UsefulLInks.svelte';
	import Word from './Word.svelte';
	import Lessons from '$lib/components/lesson/Lessons.svelte';
	import { Send } from 'lucide-svelte';
	import { language, translations } from '$lib/stores';
</script>

<div
	class="hidden select-none flex-col h-full font-serif lg:text-xl xl:text-2xl px-4 md:flex w-1/5"
>
	<a
		href="https://t.me/noxlovette"
		target="_blank"
		class="px-1 py-2 my-2 lg:px-2 lg:py-4 inline-flex"
	>
		<Send class="size-6 lg:size-7 xl:size-8 mr-2 " />

		Text Me
	</a>
	<Word></Word>
	<Lessons></Lessons>
	<UsefulLInks></UsefulLInks>
</div>

<style>
	a:hover {
		@apply bg-forest-700 transition-colors duration-300 rounded-lg text-forest-100;
	}
</style>
