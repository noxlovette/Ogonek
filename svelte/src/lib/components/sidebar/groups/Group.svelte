<script lang="ts">
  let { children } = $props();
</script>

<div
  class="dark:ring-milk-900 ring-milk-200 flex flex-col rounded-lg p-2 ring-2 shadow-md lg:p-3 xl:p-4"
>
  {@render children()}
</div>
