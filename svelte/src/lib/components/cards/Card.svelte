<script lang="ts">
  let { children, styling = "", gradient = true } = $props();
</script>

<div
  class={[
    styling,
    `ring-milk-200 shadow-milk-100 hover:shadow-cacao-100 hover:ring-cacao-400 group relative flex max-h-[150px]
    flex-col overflow-hidden
    rounded-xl
    bg-white
    p-5 ring-1 shadow-lg transition-all
    duration-200 ease-out`,
  ]}
>
  {@render children()}

  {#if gradient}
    <div
      class="pointer-events-none absolute right-0 bottom-0 left-0
    h-12 bg-gradient-to-t from-white
    to-transparent"
    ></div>
  {/if}
</div>
