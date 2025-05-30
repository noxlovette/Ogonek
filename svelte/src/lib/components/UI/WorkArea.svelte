<script lang="ts">
  let { children } = $props();
</script>

<div
  class="flex h-max min-h-full max-w-full flex-1 scroll-my-6
         flex-col gap-2
         rounded-lg bg-white
         px-2
         ring-stone-300/40 md:-mt-3 md:gap-4
         dark:bg-stone-950 "
>
  {@render children()}
</div>
