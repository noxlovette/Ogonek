<script lang="ts">
  import { LargeTitle, Toolbar } from "$lib/components";

  let { data } = $props();
</script>

<svelte:head>
  <title>Privacy | Ogonek</title>
</svelte:head>
<Toolbar>
  <LargeTitle>Политика конфиденциальности</LargeTitle>
</Toolbar>
<div class="markdown">
  <!-- eslint-disable-next-line svelte/no-at-html -->
  <!-- Input is sanitized with rehype -->
  {@html data.markdown}
</div>
