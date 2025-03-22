<script lang="ts">
  import { notification, clearNotification } from "$lib/stores";
  import { fade } from "svelte/transition";
  import { Check, AlertCircle, X, Ban } from "lucide-svelte";
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
      }, 3000);
    }
  });

  onDestroy(() => {
    if (timeout) {
      clearTimeout(timeout);
    }
  });

  function handleDismiss() {
    clearNotification();
  }
</script>

{#if $notification.message}
  <div
    transition:fade={{ duration: 200 }}
    class="fixed top-4 left-1/2 z-50 flex max-w-md -translate-x-1/2 items-center gap-3 rounded-lg shadow-lg ring-1
    {$notification.type === 'success'
      ? 'bg-green-50 text-green-700 ring-green-200 dark:bg-green-900/30 dark:ring-green-800'
      : $notification.type === 'error'
        ? 'bg-red-50 text-red-700 ring-red-200 dark:bg-red-900/30 dark:ring-red-800'
        : 'bg-cacao-50 text-cacao-700 ring-cacao-200 dark:ring-cacao-800 dark:bg-cacao-900/30'} 
    px-4 py-3"
  >
    {#if $notification.type === "success"}
      <Check class="size-5" />
    {:else if $notification.type === "error"}
      <Ban class="size-5" />
    {:else}
      <AlertCircle class="size-5" />
    {/if}

    <p class="text-sm font-medium">
      {$notification.message}
    </p>

    <button
      onclick={handleDismiss}
      class="ml-1 rounded-full p-1 transition-colors hover:bg-stone-200/50 dark:hover:bg-stone-800/50"
      aria-label="Dismiss notification"
    >
      <X class="size-4" />
    </button>
  </div>
{/if}
