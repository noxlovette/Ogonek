<script lang="ts">
  import type { PageData } from "./$types";
  import { H1, H2, DeckCard } from "$lib/components";

  let { data }: { data: PageData } = $props();
  let { decks } = data;
</script>

<H1>Decks</H1>

<section class="space-y-4">
  <H2>Recent Decks</H2>
  <div class="grid grid-cols-1 gap-4 md:grid-cols-2">
    {#each decks as deck}
      <DeckCard {deck} />
    {/each}
  </div>
</section>

<svelte:head>
  <title>Words</title>
</svelte:head>
