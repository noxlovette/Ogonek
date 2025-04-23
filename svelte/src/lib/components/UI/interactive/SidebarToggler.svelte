<script lang="ts">
  import { sidebar } from "$lib/stores";
  import { ChevronRight, ChevronLeft } from "lucide-svelte";
</script>

<button
  onclick={() => sidebar.toggle()}
  class="bg-cacao-500 hover:bg-cacao-50 hover:text-cacao-500 text-cacao-50 absolute -top-2 right-0 z-10 hidden rounded-full p-1
        shadow-sm transition-colors md:block dark:bg-stone-800
        dark:text-stone-200 dark:hover:bg-stone-700"
  aria-label={$sidebar ? "Expand sidebar" : "Collapse sidebar"}
>
  {#if $sidebar}
    <ChevronRight size={16} />
  {:else}
    <ChevronLeft size={16} />
  {/if}
</button>
