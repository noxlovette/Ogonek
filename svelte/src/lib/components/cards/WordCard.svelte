<script lang="ts">
  import type { Card } from "$lib/types";

  let {
    toggleCard,
    card,
    flippedCards = $bindable(),
  }: {
    toggleCard: (cardId: string) => void;
    card: Card;
    flippedCards: Set<string>;
  } = $props();
</script>

<div
  class="relative h-40 cursor-pointer"
  onclick={() => toggleCard(card.id)}
  onkeydown={(e) => e.key === "Enter" && toggleCard(card.id)}
  role="button"
  tabindex="0"
>
  <div
    class="ring-cacao-200 dark:ring-milk-800 absolute inset-0 h-full w-full rounded-lg bg-white p-4 shadow-sm ring backface-hidden dark:bg-stone-800 {flippedCards.has(
      card.id,
    )
      ? 'opacity-0'
      : 'opacity-100'} transition-opacity duration-300"
  >
    <div class="flex h-full items-center justify-center">
      <p class="text-center break-words">{card.front}</p>
      {#if card.media_url}
        <img
          src={card.media_url}
          alt="Card media"
          class="absolute top-2 right-2 h-8 w-8 rounded object-cover"
        />
      {/if}
    </div>
  </div>

  <div
    class="bg-cacao-50 ring-cacao-500 dark:ring-milk-800 dark:bg-cacao-900 absolute inset-0 h-full w-full rounded-lg p-4 shadow-sm ring backface-hidden {flippedCards.has(
      card.id,
    )
      ? 'opacity-100'
      : 'opacity-0'} transition-opacity duration-300"
  >
    <div class="flex h-full items-center justify-center">
      <p class="text-center break-words">{card.back}</p>
    </div>
  </div>
</div>
