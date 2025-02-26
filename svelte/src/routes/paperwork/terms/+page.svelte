<script lang="ts">
  let { data } = $props();
</script>

<svelte:head>
  <title>Terms | Firelight</title>
</svelte:head>

<div class="markdown">
  {@html data.markdown}
</div>
