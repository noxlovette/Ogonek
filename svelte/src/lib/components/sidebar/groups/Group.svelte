<script lang="ts">
  let { children } = $props();
</script>

<div
  class="flex h-max flex-col space-y-3 rounded-l-lg bg-stone-50/30 p-2 shadow-sm ring ring-stone-300/40 lg:p-3 xl:p-4 dark:bg-stone-900/30 dark:ring-stone-600/50"
>
  {@render children()}
</div>
