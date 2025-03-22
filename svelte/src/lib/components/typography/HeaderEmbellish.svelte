<script>
  let { children } = $props();
</script>

<div
  class="flex items-center justify-between rounded-lg bg-stone-50 p-4 shadow-sm ring ring-stone-200 dark:bg-stone-900 dark:ring-stone-900"
>
  {@render children?.()}
</div>
