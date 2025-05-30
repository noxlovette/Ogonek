<script lang="ts">
  let { children } = $props();
</script>

<kbd
  class="ring-default flex max-h-max min-w-max items-center rounded-md bg-stone-100 p-2 text-xs font-bold shadow-sm dark:bg-stone-900"
>
  {@render children?.()}
</kbd>
