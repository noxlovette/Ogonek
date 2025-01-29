<script lang="ts">
	import { fade } from 'svelte/transition';
	import { isSearchOpen } from '$lib/stores';

	function close() {
		isSearchOpen.set(false);
	}
</script>

{#if $isSearchOpen}
	<button
		class="fixed inset-0 dark:bg-milk-950/80 bg-milk-900/80 backdrop-blur-sm items-start pt-20 justify-center z-30"
		transition:fade={{ duration: 100 }}
		onclick={close}
		aria-label="Search Overlay"
	></button>
{/if}
