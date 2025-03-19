<script lang="ts">
  let { children } = $props();
</script>

<div
  class="flex h-max flex-col rounded-l-lg bg-stone-50 p-2 shadow-sm ring ring-stone-200 lg:p-3 xl:p-4 dark:bg-stone-950 dark:ring-stone-900"
>
  {@render children()}
</div>
