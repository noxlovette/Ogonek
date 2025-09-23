<script lang="ts">
  const { children } = $props();
</script>

<div class="bg-default ring-default rounded-xl p-2">
  {@render children()}
</div>
