<script lang="ts">
  let { seen = true } = $props();
</script>

{#if !seen}
  <block
    class="absolute -top-1 -right-1 flex aspect-square size-3 rounded-full bg-red-500 ring-1 ring-red-600"
  >
  </block>
{/if}
