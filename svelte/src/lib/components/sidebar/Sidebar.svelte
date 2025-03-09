<script lang="ts">
  import { sidebar } from "$lib/stores";
  let { elements }: { elements: ConstructorOfATypedSvelteComponent[] } =
    $props();
</script>

<div
  class={`
   bg-cacao-600 text-cacao-50 relative my-2 h-max flex-shrink-0
   flex-col rounded-lg shadow-md ring ring-stone-200 transition-all ease-in-out
   dark:bg-stone-900 dark:ring-stone-900
   ${$sidebar ? "w-24" : "w-1/5"}
   hidden md:flex md:text-lg lg:text-xl xl:text-2xl
 `}
>
  <!-- Toggle Button (Absolute Positioned) -->

  <div class="w-full overflow-hidden py-4">
    <ul
      class={`
       flex flex-col 
       ${$sidebar ? "items-center space-y-6" : "divide-cacao-400 space-y-2 divide-y-2 divide-dashed px-4 lg:space-y-3 xl:space-y-4 dark:divide-stone-600/80"}
     `}
    >
      {#each elements as Element}
        <Element />
      {/each}
    </ul>
  </div>
</div>
