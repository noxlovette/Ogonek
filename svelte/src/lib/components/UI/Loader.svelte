<script>
  import { fade } from "svelte/transition";
  import { Loader2 } from "lucide-svelte";
  import { navigating } from "$app/state";
</script>

{#if navigating.to}
  <div
    class="fixed right-10 bottom-10 z-30 flex items-center space-x-3 rounded-lg px-4 py-2 shadow-lg ring backdrop-blur-md transition-all duration-300 dark:bg-stone-900/80 dark:ring-stone-800"
    transition:fade={{ duration: 300 }}
  >
    <Loader2 class="h-6 w-6 animate-spin text-white"></Loader2>
    <p class="text-sm font-medium text-white italic">Loading...</p>
  </div>
{/if}
