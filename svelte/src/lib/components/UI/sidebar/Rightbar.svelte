<script lang="ts">
  import type { Component } from "svelte";

  let { elements }: { elements: Component[] } = $props();
</script>

<div
  class={`
      relative
      order-last
     -mr-8 
     flex flex-shrink-0
     flex-col
   `}
>
  <ul
    class={`
         flex h-full
         flex-col
         items-stretch justify-stretch
        gap-4 xl:gap-6
       `}
  >
    {#each elements as Element, index (index)}
      <Element />
    {/each}
  </ul>
</div>
