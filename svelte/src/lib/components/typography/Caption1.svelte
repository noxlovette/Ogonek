<script lang="ts">
  let { children, styling = "text-stone-500" } = $props();
</script>

<p class="text-xs font-medium {styling}">
  {@render children?.()}
</p>
