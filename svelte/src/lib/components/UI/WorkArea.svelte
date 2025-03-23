<script lang="ts">
  let { children } = $props();
</script>

<div
  class="relative flex h-max
min-h-full
max-w-full
  flex-1 scroll-my-6 flex-col gap-2 overflow-auto rounded-lg bg-white
  p-3 shadow-sm ring ring-stone-200 md:-translate-y-3
  md:gap-4 dark:bg-stone-950
dark:ring-stone-900
  "
>
  {@render children()}
</div>
