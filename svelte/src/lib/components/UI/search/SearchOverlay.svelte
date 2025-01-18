<script lang="ts">
	import { fade } from 'svelte/transition';
	import { isSearchOpen } from '$lib/stores';

	function close() {
		isSearchOpen.set(false);
	}
</script>

{#if $isSearchOpen}
	<div
		class="fixed flex inset-0 bg-milk-900/80 backdrop-blur-sm items-start pt-20 justify-center z-30"
		transition:fade={{ duration: 100 }}
		onclick={close}
		role="dialog"
		aria-label="Search Overlay"
		aria-modal="true"
	></div>
{/if}
