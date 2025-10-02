<script lang="ts">
  const { children } = $props();
</script>

<div
  class="bg-default flex items-center gap-2 border-b-4 border-stone-100 px-2 py-2.5 dark:border-stone-800"
>
  {@render children()}
</div>
