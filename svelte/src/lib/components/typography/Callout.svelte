<script lang="ts">
  let { children, styling = "" } = $props();
</script>

<p class="text-base font-medium text-stone-600 dark:text-stone-200 {styling}">
  {@render children?.()}
</p>
