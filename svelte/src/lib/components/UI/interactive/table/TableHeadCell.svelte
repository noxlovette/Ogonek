<script lang="ts">
  const { children } = $props();
</script>

<div
  class="group px-4 py-3 text-left text-sm font-semibold text-stone-700 dark:text-stone-300"
  tabindex={-1}
  role="columnheader"
  aria-sort="none"
>
  {@render children()}
</div>
