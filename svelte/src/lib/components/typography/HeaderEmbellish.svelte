<script>
  let { children } = $props();
</script>

<div
  class="flex items-center justify-between rounded-lg bg-stone-50 p-4 shadow-sm ring ring-stone-300/40 dark:bg-stone-900 dark:ring-stone-600/50"
>
  {@render children?.()}
</div>
