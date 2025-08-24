<script lang="ts">
  let { children } = $props();
</script>

<div class="flex h-full items-end gap-4">
  {@render children()}
</div>
