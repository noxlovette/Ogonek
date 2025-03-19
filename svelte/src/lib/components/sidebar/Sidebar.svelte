<script lang="ts">
  import { sidebar } from "$lib/stores";
  import { ChevronLeft, ChevronRight } from "lucide-svelte";

  let { elements }: { elements: ConstructorOfATypedSvelteComponent[] } =
    $props();
</script>

<div
  class={`
   bg-cacao-600 text-cacao-50 relative my-2 h-max flex-shrink-0 -translate-x-4
   flex-col rounded-r-lg shadow-sm ring ring-stone-200 transition-all ease-in-out
   dark:bg-stone-900 dark:ring-stone-900
   ${$sidebar ? "w-1/12" : "w-1/6"}
   hidden md:flex md:text-lg lg:text-xl xl:text-2xl
 `}
>
  <div class="w-full overflow-hidden py-2">
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
    <ul
      class={`
    flex flex-col space-y-5 py-3 ${$sidebar ? "items-center space-y-6 py-2" : " divide-stone-200/40 px-3 dark:divide-stone-600/80"}
  `}
    >
      {#each elements as Element, i}
        <Element />
      {/each}
    </ul>
  </div>
</div>
