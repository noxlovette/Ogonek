<script lang="ts">
  let { data } = $props();
</script>

<svelte:head>
  <title>Privacy | Ogonek</title>
</svelte:head>

<div class="markdown">
  <!-- eslint-disable-next-line svelte/no-at-html -->
  <!-- Input is sanitized with rehype -->
  {@html data.markdown}
</div>
