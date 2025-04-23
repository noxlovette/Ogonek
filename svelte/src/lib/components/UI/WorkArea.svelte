<script lang="ts">
  let { children } = $props();
</script>

<div
  class="flex h-max min-h-full max-w-full flex-1 scroll-my-6
         flex-col gap-2
         rounded-lg bg-white
         p-3 shadow-sm ring
         ring-stone-300/30 md:-mt-3 md:gap-4
         dark:bg-stone-950 dark:ring-stone-600/30"
>
  {@render children()}
</div>
