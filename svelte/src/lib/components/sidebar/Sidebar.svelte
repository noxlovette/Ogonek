<script lang="ts">
  let { elements }: { elements: ConstructorOfATypedSvelteComponent[] } =
    $props();
</script>

<div
  class="bg-cacao-600 text-cacao-50 dark:bg-milk-900 dark:ring-milk-900 ring-milk-200 hidden h-max w-1/5 flex-shrink-0 flex-col rounded-lg px-4 py-2
  font-serif shadow-md ring md:flex md:text-lg lg:text-xl xl:text-2xl"
>
  <ul
    class="divide-cacao-400 dark:divide-milk-600 flex flex-col space-y-2 divide-y-2 lg:space-y-3 xl:space-y-4"
  >
    {#each elements as Element}
      <Element />
    {/each}
  </ul>
</div>
