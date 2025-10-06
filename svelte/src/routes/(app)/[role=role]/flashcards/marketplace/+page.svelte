<script lang="ts">
  import { DeckCard, EmptySpace, LargeTitle, Toolbar } from "$lib/components";
  import texts from "$lib/texts.js";

  let { data } = $props();
</script>

<Toolbar>
  <LargeTitle>Деки для всех</LargeTitle>
</Toolbar>
{#if data.decks.length > 0}
  <grid class="grid gap-4 md:grid-cols-3">
    {#each data.decks as deck (deck.id)}
      <DeckCard {deck} />
    {/each}
  </grid>
{:else}
  <EmptySpace>{texts.table.empty}</EmptySpace>
{/if}
