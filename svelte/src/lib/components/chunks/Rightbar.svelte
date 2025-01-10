<script lang="ts">
	import UsefulLInks from './UsefulLInks.svelte';
	import Word from './Word.svelte';
	import Lessons from '$lib/components/lesson/Lessons.svelte';
	import { Send } from 'lucide-svelte';
	import { language, translations } from '$lib/stores';
</script>

<div
	class="hidden select-none flex-col h-full font-serif lg:text-xl xl:text-2xl md:flex w-1/5"
>
	<Word></Word>
	<Lessons></Lessons>
	<UsefulLInks></UsefulLInks>
</div>

<style>
	a:hover {
		@apply transition-all duration-300 text-brick-400 translate-x-1;
	}
</style>
