<script lang="ts">
	import type { LayoutServerData } from './$types';

	let { data, children } = $props();
</script>

{@render children()}

<svelte:head>
	<title>Lesson</title>
</svelte:head>