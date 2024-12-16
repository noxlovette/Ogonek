<script lang="ts">
	import UsefulLInks from "./UsefulLInks.svelte";
	import Word from "./Word.svelte";
	import Lessons from "./Lessons.svelte";
	import { Send } from "lucide-svelte";

</script>


	<div class="flex flex-col h-screen text-sand-100 font-serif text-2xl w-1/4 space-y-4 px-6 py-4">
		<a href="https://t.me/noxlovette" target="_blank" class="px-2 py-4 inline-flex">
			<Send class="size-8 mr-2" />
			
			Text Danila</a>
		<Word></Word>
		<Lessons></Lessons>
		<UsefulLInks></UsefulLInks>
	</div>

	
	<style>
		a:hover {
			@apply bg-sand-900/20 transition-colors duration-300 rounded-lg text-sand-100;
		}
	</style>