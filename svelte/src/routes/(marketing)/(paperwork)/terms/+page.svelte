<script lang="ts">
  import { LargeTitle, Toolbar } from "$lib/components";

  let { data } = $props();
</script>

<svelte:head>
  <title>Terms | Ogonek</title>
</svelte:head>

<Toolbar>
  <LargeTitle>Пользовательское соглашение</LargeTitle>
</Toolbar>
<div class="markdown">
  <!-- eslint-disable-next-line svelte/no-at-html -->
  <!-- Input is sanitized with rehype -->
  {@html data.markdown}
</div>
