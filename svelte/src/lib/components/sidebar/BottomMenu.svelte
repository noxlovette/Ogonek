<script lang="ts">
  let props = $props();
</script>

<div
  class="{props.class} bg-cacao-600 text-cacao-50 fixed bottom-3 left-1/2 z-10 flex w-11/12 -translate-x-1/2 items-center rounded-2xl px-2 py-4 shadow-xl ring ring-stone-200 backdrop-blur-lg md:hidden dark:bg-stone-900 dark:text-inherit dark:ring-stone-900"
>
  <ul class="{props.subclass} flex w-full items-end justify-around">
    {#each props.elements as Element, i}
      <li class="transition-all duration-300 ease-out">
        <Element />
      </li>
    {/each}
  </ul>
</div>

<style>
  li {
    transform-origin: bottom;
    touch-action: none;
  }
</style>
