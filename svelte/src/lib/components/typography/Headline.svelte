<script lang="ts">
  let { children, styling = "" } = $props();
</script>

<h4 class="text-lg font-semibold tracking-normal text-balance {styling}">
  {@render children?.()}
</h4>
