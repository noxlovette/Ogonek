<script lang="ts">
  import type { Component } from "svelte";
  import VStack from "../../VStack.svelte";

  interface MobileMenuProps {
    elements: Component[];
  }

  let { elements = [] }: MobileMenuProps = $props();
</script>

<nav
  class="ring-default fixed bottom-0 left-0 w-full bg-stone-100 pt-4 pb-8 md:hidden dark:bg-stone-800"
>
  <div class="flex justify-around">
    {#each elements as Element, index (index)}
      <Element />
    {/each}
  </div>
</nav>
