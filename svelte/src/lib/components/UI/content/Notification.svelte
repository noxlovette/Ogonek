<script lang="ts">
  import { notification, clearNotification } from "$lib/stores";
  import { fade } from "svelte/transition";
  import { Check, AlertCircle, X, Ban } from "@lucide/svelte";
  import { onDestroy } from "svelte";

  let timeout: ReturnType<typeof setTimeout> | null = null;

  $effect(() => {
    if ($notification.message) {
      if (timeout) clearTimeout(timeout);
      timeout = setTimeout(() => clearNotification(), 3000);
    }
  });

  onDestroy(() => {
    if (timeout) clearTimeout(timeout);
  });

  function handleDismiss() {
    clearNotification();
  }
</script>

{#if $notification.message}
  <div
    transition:fade={{ duration: 250 }}
    class="bg-default ring-default fixed top-5 left-1/2 z-50 flex max-w-sm -translate-x-1/2 items-center gap-4 rounded-3xl px-5 py-3 shadow-xl backdrop-blur-md
"
  >
    {#if $notification.type === "success"}
      <Check class="size-5 text-emerald-500" />
    {:else if $notification.type === "error"}
      <Ban class="size-5 text-rose-500" />
    {:else}
      <AlertCircle class="size-5 text-yellow-500" />
    {/if}

    <p
      role="alert"
      aria-label="Notification"
      class="flex-1 text-sm font-medium text-stone-800 dark:text-stone-100"
    >
      {$notification.message}
    </p>

    <button
      onclick={handleDismiss}
      class="rounded-full p-1 hover:bg-stone-200/40 dark:hover:bg-stone-800/40"
      aria-label="Dismiss notification"
    >
      <X class="size-4 text-stone-600 dark:text-stone-300" />
    </button>
  </div>
{/if}
