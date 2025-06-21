<script lang="ts">
  let { children } = $props();
</script>

<div
  class="flex h-max min-h-full max-w-full flex-1 scroll-my-6
         flex-col gap-2
         rounded-lg bg-white
         ring-stone-300/40
         md:gap-4 md:px-2
         dark:bg-stone-950"
>
  {@render children()}
</div>
