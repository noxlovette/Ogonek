<script lang="ts">
  import type { Component, Snippet } from "svelte";
  let { elements, children }: { elements: Component[]; children?: Snippet } =
    $props();
</script>

<div
  class={`
    ring-default bg-default bg-default relative z-40 my-2
    flex h-max w-full
    flex-shrink-0 flex-col rounded-2xl
    shadow-lg 
    md:text-lg lg:text-xl
    xl:text-2xl
  `}
>
  <div class="w-full overflow-hidden py-2">
    <ul
      class="flex flex-col gap-4 divide-stone-200/40 px-3 dark:divide-stone-600/80"
    >
      {#each elements as Element, index (index)}
        <Element />
      {/each}
      {@render children?.()}
    </ul>
  </div>
</div>
