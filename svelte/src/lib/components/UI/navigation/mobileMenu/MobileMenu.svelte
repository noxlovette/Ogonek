<script lang="ts">
  import type { Component } from "svelte";

  interface MobileMenuProps {
    elements: Component[];
  }

  let { elements = [] }: MobileMenuProps = $props();
</script>

<nav
  class="ring-default fixed bottom-2 left-1/2 z-50 h-20 w-[90vw] max-w-xl -translate-x-1/2 rounded-full bg-stone-100/50 p-2 backdrop-blur-sm md:hidden"
>
  <div class="flex justify-around">
    {#each elements as Element, index (index)}
      <Element />
    {/each}
  </div>
</nav>
