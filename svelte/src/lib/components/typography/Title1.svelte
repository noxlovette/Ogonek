<script lang="ts">
  let { children, styling = "" } = $props();
</script>

<h1 class="text-2xl font-semibold tracking-tight md:text-4xl {styling}">
  {@render children?.()}
</h1>
