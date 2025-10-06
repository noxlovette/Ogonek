<script lang="ts">
  let { children, styling = "" } = $props();
</script>

<div class="flex h-max min-h-full max-w-full flex-1 flex-col gap-2 md:gap-4">
  {@render children()}
</div>
