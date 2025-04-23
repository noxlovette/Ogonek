<script lang="ts">
  let { children } = $props();
</script>

<div
  class="flex h-max flex-col rounded-l-lg bg-stone-50 p-2 shadow-sm ring ring-stone-300/30 lg:p-3 xl:p-4 dark:bg-stone-900 dark:ring-stone-600/30"
>
  {@render children()}
</div>
