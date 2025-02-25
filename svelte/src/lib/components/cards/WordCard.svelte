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

  const truncateIfNeeded = (text: string) => {
    return text.length > 120
      ? "text-sm"
      : text.length > 80
        ? "text-base"
        : "text-lg";
  };
</script>

<div
  class="perspective-1000 relative h-48 transform-gpu cursor-pointer"
  onclick={() => toggleCard(card.id)}
  onkeydown={(e) => e.key === "Enter" && toggleCard(card.id)}
  role="button"
  tabindex="0"
  aria-label={flippedCards.has(card.id)
    ? "Show front of card"
    : "Show back of card"}
>
  <!-- Card Front -->
  <div
    class="ring-cacao-200 dark:ring-milk-800 dark:bg-milk-900 absolute inset-0 flex h-full w-full items-center justify-center rounded-lg bg-white p-5 shadow-md ring-1 transition-all duration-300 backface-hidden hover:shadow-lg {flippedCards.has(
      card.id,
    )
      ? 'opacity-0'
      : 'opacity-100'}"
  >
    <div class="flex h-full w-full flex-col items-center justify-center">
      <p
        class="max-w-full overflow-auto text-center font-medium break-words {truncateIfNeeded(
          card.front,
        )}"
      >
        {card.front}
      </p>

      {#if card.media_url}
        <div class="absolute top-2 right-2">
          <img
            src={card.media_url}
            alt="Card media"
            class="h-10 w-10 rounded-md object-cover shadow-sm"
          />
        </div>
      {/if}

      <div class="absolute right-3 bottom-2">
        <span class="text-milk-400 dark:text-milk-500 text-xs"
          >Click to flip</span
        >
      </div>
    </div>
  </div>

  <!-- Card Back -->
  <div
    class="bg-cacao-50 ring-cacao-500 dark:ring-milk-700 dark:bg-milk-900 absolute inset-0 flex h-full w-full items-center justify-center rounded-lg p-5 shadow-md ring-1 transition-all duration-300 backface-hidden hover:shadow-lg {flippedCards.has(
      card.id,
    )
      ? 'opacity-100'
      : 'opacity-0'}"
  >
    <div class="flex h-full w-full flex-col items-center justify-center">
      <p
        class="max-w-full overflow-auto text-center font-medium break-words {truncateIfNeeded(
          card.back,
        )}"
      >
        {card.back}
      </p>

      <div class="absolute right-3 bottom-2">
        <span class="text-milk-400 dark:text-milk-700 text-xs"
          >Click to flip</span
        >
      </div>
    </div>
  </div>

  <!-- Flip Indicator -->
  <div class="absolute -top-1 -right-1 z-10 h-8 w-8 overflow-hidden">
    <div
      class="absolute top-0 right-0 h-2 w-2 rounded-full transition-colors {flippedCards.has(
        card.id,
      )
        ? 'bg-cacao-500 dark:bg-cacao-400'
        : 'bg-milk-200 dark:bg-milk-700'}"
    ></div>
  </div>
</div>

<style>
  .perspective-1000 {
    perspective: 1000px;
  }

  .backface-hidden {
    backface-visibility: hidden;
  }
</style>
