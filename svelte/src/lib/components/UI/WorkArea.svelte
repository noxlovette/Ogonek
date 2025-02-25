<script lang="ts">
  import { page } from "$app/state";
  let { children } = $props();
  import { rightbar, sidebar } from "$lib/stores";
  import { ChevronLeft, ChevronRight } from "lucide-svelte";

  const role = page.params.role;
</script>

<div
  class="ring-milk-200 dark:ring-milk-900 dark:bg-milk-950
  relative mx-4 flex flex-1 scroll-my-6 flex-col gap-4
  rounded-lg bg-white px-5 py-3 pb-24 shadow-md ring md:mr-0 md:h-full md:px-6 md:pb-3
  lg:px-8 lg:py-4"
>
  {@render children()}

  <button
    onclick={() => sidebar.toggle()}
    class="bg-cacao-500 dark:bg-milk-800 dark:text-milk-200 hover:bg-cacao-400 dark:hover:bg-milk-700 absolute top-4 -left-3 z-10
            hidden rounded-full p-1 text-white shadow-md
            transition-colors md:block"
    aria-label={$sidebar ? "Expand sidebar" : "Collapse sidebar"}
  >
    {#if $sidebar}
      <ChevronRight size={16} />
    {:else}
      <ChevronLeft size={16} />
    {/if}
  </button>

  {#if role === "s"}
    <button
      onclick={() => rightbar.toggle()}
      class="bg-cacao-500 dark:bg-milk-800 dark:text-milk-200 hover:bg-cacao-400 dark:hover:bg-milk-700
          absolute top-4 -right-3 z-30 rounded-full p-1 text-white
          shadow-md transition-colors"
      aria-label={$rightbar ? "Expand rightbar" : "Collapse rightbar"}
    >
      {#if $rightbar}
        <ChevronLeft size={16} />
      {:else}
        <ChevronRight size={16} />
      {/if}
    </button>
  {/if}
</div>
