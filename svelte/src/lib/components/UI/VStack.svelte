<script lang="ts">
  let { children, styling = "" } = $props();
</script>

<div class="flex h-full items-end gap-4 {styling}">
  {@render children()}
</div>
