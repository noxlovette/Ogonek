<script lang="ts">
  import { LargeTitle, Toolbar } from "$lib/components";
  import type { PageProps } from "./$types";

  let { data }: PageProps = $props();
</script>

<Toolbar>
  <LargeTitle>
    Preview {data.publicContent.title}
  </LargeTitle>
</Toolbar>
<div class="markdown">
  {@html data.rendered}
</div>

<svelte:head>
  <title>
    {data.publicContent.title}
  </title>
</svelte:head>
