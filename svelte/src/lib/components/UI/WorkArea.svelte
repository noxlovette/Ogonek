<script lang="ts">
  import { page } from "$app/state";
  let { children } = $props();
  import { rightbar, sidebar } from "$lib/stores";
  import { ChevronLeft, ChevronRight } from "lucide-svelte";

  const role = page.params.role;
</script>

<div
  class="relative mx-6 flex h-max
  min-h-full flex-1 -translate-y-3 scroll-my-6 flex-col gap-4 rounded-lg
  bg-white px-5 py-3 pb-24 shadow-sm ring ring-stone-200 md:mr-0 md:px-5 md:pb-3 lg:py-4
  dark:bg-stone-950 dark:ring-stone-900
  
  "
>
  {@render children()}

  <button
    onclick={() => sidebar.toggle()}
    class="bg-cacao-500 hover:bg-cacao-50 absolute top-5 -left-10 z-10 hidden rounded-full p-1 text-white
            shadow-sm transition-colors hover:text-inherit md:block dark:bg-stone-800
            dark:text-stone-200 dark:hover:bg-stone-700"
    aria-label={$sidebar ? "Expand sidebar" : "Collapse sidebar"}
  >
    {#if $sidebar}
      <ChevronRight size={16} />
    {:else}
      <ChevronLeft size={16} />
    {/if}
  </button>
</div>
