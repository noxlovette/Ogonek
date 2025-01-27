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
		<Check
			class="size-5 lg:size-6 p-1 bg-milk-100 dark:bg-inherit dark:ring-1 dark:ring-milk-900 text-pakistan-700 rounded-full"
		/>
	{:else if type === 'error'}
		<X
			class="size-5 lg:size-6 p-1 bg-milk-100 dark:bg-inherit dark:ring-1 dark:ring-milk-900  text-red-700 rounded-full"
		/>
	{:else}
		<AlertCircle
			class="size-5 lg:size-6 p-1 bg-milk-100 dark:bg-inherit dark:ring-1 dark:ring-milk-900  text-brick-700 rounded-full"
		/>
	{/if}
{/snippet}

{#if $notification.message}
	<div
		transition:fly={{
			duration: 300,
			easing: quintInOut,
			x: 0,
			y: 100
		}}
		class="fixed items-center left-1/2 -translate-x-1/2 bottom-5 min-w-[200px] z-50 max-w-md dark:bg-milk-950 bg-milk-50
			shadow-md rounded-full flex gap-6 px-4 py-2 ring-1 {$notification.type === 'success'
			? 'ring-pakistan-700'
			: $notification.type === 'error'
				? 'ring-red-700'
				: 'ring-amber-700'}"
	>
		{@render icon($notification.type)}

		<p
			class="flex text-sm font-bold

		text-milk-800 dark:text-inherit
		"
		>
			{$notification.message}
		</p>
	</div>
{/if}
