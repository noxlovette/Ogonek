<script lang="ts">
  let { children } = $props();
</script>

<div class="flex size-full flex-col p-12">
  {@render children()}
</div>
