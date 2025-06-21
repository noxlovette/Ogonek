<script lang="ts">
  let { children } = $props();
</script>

<div class="flex flex-1 flex-col items-center overflow-auto">
  {@render children?.()}
</div>
