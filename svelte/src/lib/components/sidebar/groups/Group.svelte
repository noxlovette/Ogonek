<script lang="ts">
  let { children } = $props();
</script>

<div
  class="flex flex-col rounded-l-lg p-2 shadow-sm ring ring-stone-200 lg:p-3 xl:p-4 dark:ring-stone-900"
>
  {@render children()}
</div>
