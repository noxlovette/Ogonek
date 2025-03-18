<script lang="ts">
  import { sidebar } from "$lib/stores";
  let { elements }: { elements: ConstructorOfATypedSvelteComponent[] } =
    $props();
</script>

<div
  class={`
   bg-cacao-600 text-cacao-50 relative my-2 -ml-4 h-max flex-shrink-0
   flex-col rounded-r-lg shadow-sm ring ring-stone-200 transition-all ease-in-out
   dark:bg-stone-900 dark:ring-stone-900
   ${$sidebar ? "w-1/12" : "w-1/6"}
   hidden md:flex md:text-lg lg:text-xl xl:text-2xl
 `}
>
  <div class="w-full overflow-hidden py-2">
    <ul
      class={`
    flex flex-col space-y-2 ${$sidebar ? "items-center space-y-6 py-2" : "divide-y divide-stone-200/40 px-3 dark:divide-stone-600/80"}
  `}
    >
      {#each elements as Element, i}
        <Element />
      {/each}
    </ul>
  </div>
</div>
