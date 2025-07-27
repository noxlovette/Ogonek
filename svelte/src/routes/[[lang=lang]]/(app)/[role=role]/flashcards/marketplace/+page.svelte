<script lang="ts">
  import { DeckCard, EmptySpace, H1, HeaderEmbellish } from "$lib/components";
  import { m } from "$lib/paraglide/messages";

  let { data } = $props();
</script>

<HeaderEmbellish>
  <H1>Public Decks</H1>
</HeaderEmbellish>
{#if data.decks.length > 0}
  <grid class="grid gap-4 md:grid-cols-3">
    {#each data.decks as deck (deck.id)}
      <DeckCard {deck} />
    {/each}
  </grid>
{:else}
  <EmptySpace>{m.noFlashcards()}</EmptySpace>
{/if}
