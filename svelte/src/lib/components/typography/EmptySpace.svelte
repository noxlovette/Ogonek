<script lang="ts">
  let { children } = $props();
</script>

<div
  class="flex h-40 w-full flex-col items-center justify-center rounded-lg border-2 border-dashed border-stone-200 p-4 text-center dark:border-stone-700"
>
  {@render children?.()}
</div>
