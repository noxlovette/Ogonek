<script lang="ts">
	import Header2 from '$lib/components/typography/Header2.svelte';
	import { Forvo, Dictionary, Thesaurus } from '$lib/components/sidebar/elements';
	import Group from './Group.svelte';

	let elements = [Forvo, Dictionary, Thesaurus];
</script>

<Group>
	<Header2>Useful Links</Header2>

	{#each elements as Element}
		<Element />
	{/each}
</Group>
