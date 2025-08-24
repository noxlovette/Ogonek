<script lang="ts">
  let { children, styling = "" } = $props();
</script>

<p class="text-base font-medium text-stone-600 {styling}">
  {@render children?.()}
</p>
