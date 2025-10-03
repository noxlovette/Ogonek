<script lang="ts">
  const { children } = $props();
</script>

<div
  class="px-4 py-3 text-sm text-stone-600 duration-200 ease-in-out group-hover:text-stone-900 dark:text-stone-400 dark:group-hover:text-stone-200"
>
  {@render children()}
</div>
