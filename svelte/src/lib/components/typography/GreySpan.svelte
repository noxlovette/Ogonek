<script lang="ts">
  let { children } = $props();
</script>

<span
  class="rounded-full bg-stone-100 px-3 py-1 text-sm font-medium text-stone-600 dark:bg-stone-700 dark:text-stone-300"
>
  {@render children?.()}
</span>
