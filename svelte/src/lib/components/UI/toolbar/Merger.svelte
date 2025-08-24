<script lang="ts">
  let { children } = $props();
</script>

<div class="flex gap-3 rounded-full p-1 shadow-lg">
  {@render children()}
</div>
