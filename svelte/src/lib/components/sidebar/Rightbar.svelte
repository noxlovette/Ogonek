<script lang="ts">
  let { elements }: { elements: ConstructorOfATypedSvelteComponent[] } =
    $props();
</script>

<div
  class="hidden h-max w-1/5 flex-shrink-0 flex-col rounded-lg bg-inherit px-4 font-serif md:flex md:text-lg lg:text-xl xl:text-2xl"
>
  <ul class={["flex flex-col space-y-2 lg:space-y-3 xl:space-y-4"]}>
    {#each elements as Element}
      <Element />
    {/each}
  </ul>
</div>
