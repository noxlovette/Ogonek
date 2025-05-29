<script lang="ts">
  import { rightbar } from "$lib/stores/sidebar";
  import type { Component } from "svelte";

  let { elements }: { elements: Component[] } = $props();
</script>

<div
  class={`
      relative
      -mr-8
     flex-shrink-0 -translate-y-3
     flex-col
     transition-all duration-150 ease-in-out
     ${$rightbar ? "w-0" : "w-1/5"}
     order-last hidden
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
         ${$rightbar ? "items-center space-y-6" : " space-y-2 px-4 lg:space-y-3 xl:space-y-4"}
       `}
  >
    {#each elements as Element, index (index)}
      <Element />
    {/each}
  </ul>
</div>
