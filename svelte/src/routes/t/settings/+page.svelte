<script lang="ts">
	import Header from '$lib/components/typography/Header.svelte';
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();
</script>

<Header>Settings</Header>
