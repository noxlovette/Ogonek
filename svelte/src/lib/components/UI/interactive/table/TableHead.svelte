<script lang="ts">
  const { children } = $props();
</script>

<div class="flex items-center gap-2 bg-stone-100 px-2 py-2.5 dark:bg-stone-900">
  {@render children()}
</div>
