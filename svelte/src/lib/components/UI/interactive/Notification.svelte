<script lang="ts">
  import { notification, clearNotification } from "$lib/stores";
  import { fly } from "svelte/transition";
  import { quintInOut } from "svelte/easing";
  import { Check, AlertCircle, X } from "lucide-svelte";
  import type { Toast } from "$lib/types";
  import { onDestroy } from "svelte";

  let timeout: ReturnType<typeof setTimeout> | null = null;

  $effect(() => {
    if ($notification.message) {
      if (timeout) {
        clearTimeout(timeout);
      }
      timeout = setTimeout(() => {
        clearNotification();
      }, 2800);
    }
  });

  onDestroy(() => {
    if (timeout) {
      clearTimeout(timeout);
    }
  });
</script>

{#snippet icon(type: Toast["type"])}
  {#if type === "success"}
    <Check
      class="bg-milk-100 dark:ring-milk-900 size-5 rounded-full p-1 text-green-700 lg:size-6 dark:bg-inherit dark:ring-1"
    />
  {:else if type === "error"}
    <X
      class="bg-milk-100 dark:ring-milk-900 size-5 rounded-full p-1 text-red-700 lg:size-6  dark:bg-inherit dark:ring-1"
    />
  {:else}
    <AlertCircle
      class="bg-milk-100 dark:ring-milk-900 text-cacao-700 size-5 rounded-full p-1 lg:size-6  dark:bg-inherit dark:ring-1"
    />
  {/if}
{/snippet}

{#if $notification.message}
  <div
    transition:fly={{
      duration: 300,
      easing: quintInOut,
      x: 0,
      y: 100,
    }}
    class="dark:bg-milk-950 bg-milk-50 fixed bottom-5 left-1/2 z-50 flex max-w-md -translate-x-1/2
			items-center gap-3 rounded-full px-4 py-2 shadow-md ring-1 {$notification.type ===
    'success'
      ? 'ring-green-700'
      : $notification.type === 'error'
        ? 'ring-red-700'
        : 'ring-amber-700'}"
  >
    {@render icon($notification.type)}

    <p
      class="text-milk-800 flex text-sm
font-bold
		capitalize dark:text-inherit
		"
    >
      {$notification.message}
    </p>
  </div>
{/if}
