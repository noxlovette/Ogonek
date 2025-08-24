<script>
  let { children } = $props();
</script>

<div class="relative flex w-full items-end justify-between">
  {@render children?.()}
</div>
