<script lang="ts">
  let { children } = $props();
</script>

<div
  class="md: relative flex
h-max
  min-h-full flex-1 scroll-my-6 flex-col gap-2 rounded-lg bg-white
  px-5 py-3 pb-24 shadow-sm ring ring-stone-200 md:gap-4
  dark:bg-stone-950 dark:ring-stone-900
  "
>
  {@render children()}
</div>
