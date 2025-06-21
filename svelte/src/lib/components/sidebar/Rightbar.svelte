<script lang="ts">
  import type { Component } from "svelte";

  let { elements }: { elements: Component[] } = $props();
</script>

<div
  class={`
      relative
      order-last
     -mr-8 hidden
     flex-shrink-0 -translate-y-3 flex-col
     transition-all
     duration-150 ease-in-out
     md:flex
     md:text-lg lg:text-xl
     xl:text-2xl
   `}
>
  <ul
    class={`
         flex h-full
         flex-col
         items-stretch justify-stretch
        space-y-2 lg:space-y-3 xl:space-y-4
       `}
  >
    {#each elements as Element, index (index)}
      <Element />
    {/each}
  </ul>
</div>
