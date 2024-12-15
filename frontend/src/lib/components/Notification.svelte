<script lang="ts">
	import { notification } from '$lib/stores';
	import { onMount } from 'svelte';

	let timeout;

	$: if ($notification.message) {
		clearTimeout(timeout);
		timeout = setTimeout(() => {
			notification.set({ message: null, type: null });
		}, 2800);
	}
</script>

{#if $notification.message}
	<div
		class="fixed bottom-2 left-2 bg-forest-800 text-forest-50 text-center rounded-lg flex p-4 items-center justify-center"
	>
		<p class="text-forest-50 font-bold text-xl p-2">{$notification.message}</p>
	</div>
{/if}
