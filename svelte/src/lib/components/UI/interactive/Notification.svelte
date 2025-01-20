<script lang="ts">
	import { notification, clearNotification } from '$lib/stores';
	import { fly } from 'svelte/transition';
	import { quintInOut } from 'svelte/easing';
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
		<Check class="size-5 lg:size-7 text-pakistan-500" />
	{:else if type === 'error'}
		<X class="size-5 lg:size-7 text-red-500" />
	{:else}
		<AlertCircle class="size-5 lg:size-7 text-brick-500" />
	{/if}
{/snippet}

{#if $notification.message}
	<div
		transition:fly={{
			duration: 300,
			easing: quintInOut,
			x: 0,
			y: -400
		}}
		class="fixed items-center left-1/2 -translate-x-1/2 top-20 min-w-[200px] z-50 max-w-md bg-milk-50
			 shadow-md rounded-lg flex gap-2 p-4 ring-2
			 {$notification.type === 'success'
			? 'ring-pakistan-500'
			: $notification.type === 'error'
				? 'ring-red-500'
				: 'ring-amber-500'}"
	>
		{@render icon($notification.type)}

		<p class="text-brick-700 text-sm font-bold">
			{$notification.message}
		</p>
	</div>
{/if}
