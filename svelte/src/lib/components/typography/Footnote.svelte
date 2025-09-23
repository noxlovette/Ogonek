<script lang="ts">
  let { children, styling = "" } = $props();
</script>

<p class="text-sm text-stone-500 dark:text-stone-300 {styling}">
  {@render children?.()}
</p>
