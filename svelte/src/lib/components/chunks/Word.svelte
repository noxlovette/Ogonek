<script lang="ts">
	import { getContext } from 'svelte';
	import { language, translations } from '$lib/stores';

	const word = {
		word: 'Loading...',
		results: []
	};
</script>

<div class="flex flex-col p-2 lg:p-3 xl:p-4 ring-2 ring-milk-200 rounded-lg shadow-md">
	<h2 class=" lg:text-lg xl:text-xl">Random Word</h2>
	<div class="p-2">
		<h3 class="italic select-text">
			{word.word}
		</h3>
		<p class="text-xs lg:text-base select-text">No definition found</p>
	</div>
</div>
