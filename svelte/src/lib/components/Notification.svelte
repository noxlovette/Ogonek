<script lang="ts">
	import { run } from 'svelte/legacy';

	import { notification } from '$lib/stores';
	import { onMount } from 'svelte';
	import { fade } from 'svelte/transition';

	let timeout = $state();

	run(() => {
		if ($notification.message) {
			clearTimeout(timeout);
			timeout = setTimeout(() => {
				notification.set({ message: null, type: null });
			}, 2800);
		}
	});
</script>

{#if $notification.message}
	<div
		transition:fade
		class="fixed bottom-2 left-2 bg-sand-900 border-2 border-sand-900/20 text-sand-100 text-center rounded-lg flex p-4 items-center justify-center"
	>
		<p class=" font-bold text-xl p-2">{$notification.message}</p>
	</div>
{/if}
