<script lang="ts">
  let { data } = $props();
</script>

<svelte:head>
  <title>Privacy | Ogonek</title>
</svelte:head>

<div class="markdown">
  {@html data.markdown}
</div>
