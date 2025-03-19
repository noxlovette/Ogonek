<script lang="ts">
  let { children } = $props();
</script>

<div
  class="relative mx-6 flex h-max
  min-h-full flex-1 -translate-y-3 scroll-my-6 flex-col gap-4 rounded-lg
  bg-white px-5 py-3 pb-24 shadow-sm ring ring-stone-200 md:mr-0 md:px-5 md:pb-3 lg:py-4
  dark:bg-stone-950 dark:ring-stone-900
  "
>
  {@render children()}
</div>
