<script lang="ts">
  let { children, styling = "" } = $props();
</script>

<p class="text-xs font-medium text-stone-500 {styling}">
  {@render children?.()}
</p>
