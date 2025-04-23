<script>
  let { children } = $props();
</script>

<div
  class="flex items-center justify-between rounded-lg bg-stone-50 p-4 ring ring-stone-300/30 dark:bg-stone-900 dark:ring-stone-600/30"
>
  {@render children?.()}
</div>
