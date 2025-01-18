<script lang="ts">
	import { notification, clearNotification } from '$lib/stores';
	import { fly } from 'svelte/transition';
	import { quintOut, quintIn, quintInOut } from 'svelte/easing';
	import { Check, AlertCircle, X } from 'lucide-svelte';
	import type { Toast } from '$lib/types';
	import { onDestroy } from 'svelte';

	let timeout: ReturnType<typeof setTimeout> | null = null;

	$effect(() => {
		if ($notification.message) {
			if (timeout) {
				clearTimeout(timeout);
			}
			timeout = setTimeout(() => {
				clearNotification(); // Reset the notification store
			}, 2800);
		}
	});

	onDestroy(() => {
		if (timeout) {
			clearTimeout(timeout);
		}
	});
</script>

{#snippet icon(type: Toast['type'])}
	{#if type === 'success'}
		<Check class="w-5 h-5 text-pakistan-500" />
	{:else if type === 'error'}
		<X class="w-5 h-5 text-red-500" />
	{:else}
		<AlertCircle class="w-5 h-5 text-brick-500" />
	{/if}
{/snippet}

{#if $notification.message}
	<div
		transition:fly={{
			duration: 300,
			easing: quintInOut,
			x: 400,
			y: 0
		}}
		class="fixed bottom-4 right-4 min-w-[320px] max-w-md bg-milk-50
			 shadow-lg rounded-lg flex items-center gap-3 p-4 border-l-4
			 {$notification.type === 'success'
			? 'border-l-pakistan-500'
			: $notification.type === 'error'
				? 'border-l-red-500'
				: 'border-l-amber-500'}"
	>
		{@render icon($notification.type)}

		<p class="text-brick-700 text-sm font-bold">
			{$notification.message}
		</p>
	</div>
{/if}
