<script lang="ts">
  let { children } = $props();
</script>

<div class="space-y-4">
  {@render children?.()}
</div>
