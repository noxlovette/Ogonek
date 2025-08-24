<script lang="ts">
  let { children } = $props();
</script>

<div class="flex h-max min-h-full max-w-full flex-1 flex-col gap-2 md:gap-8">
  {@render children()}
</div>
