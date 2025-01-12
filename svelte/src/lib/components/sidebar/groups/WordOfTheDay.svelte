<script lang="ts">
	import { getContext } from 'svelte';
	import { language, translations } from '$lib/stores';
	import Group from './Group.svelte';
	import Header2 from '$lib/components/typography/Header2.svelte';

	const word = {
		word: 'Loading...',
		results: []
	};
</script>

<Group>
	<Header2>Word</Header2>
		<h3 class="italic select-text">
			{word.word}
		</h3>
		<p class="text-xs lg:text-base select-text">No definition found</p>
</Group>
	