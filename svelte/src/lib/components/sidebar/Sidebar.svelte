<script lang="ts">
  import { ChevronLeft, ChevronRight } from "lucide-svelte";
  import { sidebar } from "$lib/stores/sidebar";

  let { elements }: { elements: ConstructorOfATypedSvelteComponent[] } =
    $props();
</script>

<div
  class={`
   bg-cacao-600 text-cacao-50 dark:bg-milk-900 dark:ring-milk-900 ring-milk-200
   relative h-max flex-shrink-0 flex-col rounded-lg shadow-md ring
   transition-all ease-in-out
   ${$sidebar ? "w-24" : "w-1/5"}
   hidden md:flex md:text-lg lg:text-xl xl:text-2xl
 `}
>
  <!-- Toggle Button (Absolute Positioned) -->
  <button
    onclick={() => sidebar.toggle()}
    class="bg-cacao-500 dark:bg-milk-800 dark:text-milk-200 hover:bg-cacao-400 dark:hover:bg-milk-700 absolute top-4
            -right-3 z-10 rounded-full p-1 text-white
            shadow-md transition-colors"
    aria-label={$sidebar ? "Expand sidebar" : "Collapse sidebar"}
  >
    {#if $sidebar}
      <ChevronRight size={16} />
    {:else}
      <ChevronLeft size={16} />
    {/if}
  </button>

  <div class="w-full overflow-hidden py-4">
    <ul
      class={`
       flex flex-col 
       ${$sidebar ? "items-center space-y-6" : "divide-cacao-400 dark:divide-milk-600 space-y-2 divide-y-2 px-4 lg:space-y-3 xl:space-y-4"}
     `}
    >
      {#each elements as Element}
        <Element />
      {/each}
    </ul>
  </div>
</div>
