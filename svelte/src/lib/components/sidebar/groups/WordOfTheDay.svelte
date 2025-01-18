<script lang="ts">
	import { getContext } from 'svelte';
	import Group from './Group.svelte';
	import H2 from '$lib/components/typography/H2.svelte';

	const word = {
		word: 'Loading...',
		results: []
	};
</script>

<Group>
	<H2>Word</H2>
	<h3 class="italic select-text">
		{word.word}
	</h3>
	<p class="text-xs lg:text-base select-text">No definition found</p>
</Group>
