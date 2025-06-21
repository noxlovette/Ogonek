<script lang="ts">
  import { sidebar } from "$lib/stores";
  import SidebarToggler from "../UI/interactive/SidebarToggler.svelte";
  import type { Component, Snippet } from "svelte";

  let { elements, children }: { elements: Component[]; children?: Snippet } =
    $props();
</script>

<div
  class={`
   ring-default bg-default relative my-2 flex h-max
   w-full  flex-shrink-0 -translate-x-4 flex-col
    rounded-r-lg
shadow-sm
   transition-all ease-in-out md:text-lg lg:text-xl xl:text-2xl
 `}
>
  <div class="w-full overflow-hidden py-2">
    <SidebarToggler></SidebarToggler>
    <ul
      class={`flex flex-col gap-3 ${$sidebar ? "items-center space-y-6 py-2" : " divide-stone-200/40 px-3 dark:divide-stone-600/80"}
  `}
    >
      {#each elements as Element, index (index)}
        <Element />
      {/each}
      {@render children?.()}
    </ul>
  </div>
</div>
