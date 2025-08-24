<script>
  let { children } = $props();
</script>

<div class="relative flex w-full justify-between">
  {@render children?.()}
</div>
