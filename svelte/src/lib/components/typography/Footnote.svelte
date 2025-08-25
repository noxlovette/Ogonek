<script lang="ts">
  let { children, styling = "" } = $props();
</script>

<p class="text-sm text-stone-500 {styling}">
  {@render children?.()}
</p>
