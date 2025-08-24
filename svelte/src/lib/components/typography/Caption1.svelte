<script lang="ts">
  let { children, styling = "text-stone-500 dark:text-stone-200" } = $props();
</script>

<p class="text-xs font-medium {styling}">
  {@render children?.()}
</p>
