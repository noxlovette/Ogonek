<script lang="ts">
  import { sidebar } from "$lib/stores";
  import { ChevronRight, ChevronLeft } from "lucide-svelte";
</script>

<button
  onclick={() => sidebar.toggle()}
  class="bg-accent/30 hover:bg-accent/30 hover:text-accent text-accent
        absolute -top-2 right-0 z-10 hidden rounded-full p-1 shadow-sm
        backdrop-blur-md md:block dark:bg-stone-800/30
        dark:text-stone-200 dark:hover:bg-stone-700/30"
  aria-label={$sidebar ? "Expand sidebar" : "Collapse sidebar"}
>
  {#if $sidebar}
    <ChevronRight size={16} />
  {:else}
    <ChevronLeft size={16} />
  {/if}
</button>
