<script lang="ts">
  let { children, styling = "" } = $props();
</script>

<p class="text-xs text-stone-400 italic {styling}">
  {@render children?.()}
</p>
