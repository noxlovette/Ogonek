<script lang="ts">
  let { children } = $props();
</script>

<div class="flex w-full flex-col gap-4">
  {@render children()}
</div>
