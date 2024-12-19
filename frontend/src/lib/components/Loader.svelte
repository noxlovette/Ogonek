<script>
	import { fade } from 'svelte/transition';

	import { navigating } from '$app/stores';
</script>

{#if $navigating}
	<div
		class="fixed left-0 top-0 z-30 flex h-full w-full items-center justify-center bg-sand-900 bg-opacity-50 backdrop-blur select-none"
		transition:fade={{ duration: 300 }}
	>
		<div class="text-sand-100 text-4xl font-serif">Firelight is loading...</div>
	</div>
{/if}
