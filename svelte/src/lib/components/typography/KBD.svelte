<script lang="ts">
  let { children } = $props();
</script>

<kbd
  class="flex max-h-max min-w-max items-center rounded-md bg-stone-100 p-2 text-xs font-bold shadow-sm ring ring-stone-300/40 dark:bg-stone-900 dark:ring-stone-600/50"
>
  {@render children?.()}
</kbd>
