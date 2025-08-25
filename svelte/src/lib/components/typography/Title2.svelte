<script lang="ts">
  let { children, styling = "" } = $props();
</script>

<h2 class="text-2xl font-semibold tracking-tight md:text-3xl {styling}">
  {@render children?.()}
</h2>
