<script lang="ts">
  const { children } = $props();
</script>

<div
  class="border-b border-stone-300/30 bg-stone-50 dark:border-stone-700 dark:bg-stone-900/30"
>
  {@render children()}
</div>
