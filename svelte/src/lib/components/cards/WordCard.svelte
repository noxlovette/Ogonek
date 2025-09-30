<script lang="ts">
  import { m } from "$lib/paraglide/messages";
  import type { Card } from "$lib/types";
  import { Callout, Caption1 } from "../typography";

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
  data-cy="word-card-front"
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
    class="ring-default absolute inset-0 flex h-full w-full items-center justify-center rounded-2xl bg-white p-5 shadow-sm duration-150 backface-hidden hover:shadow-sm dark:bg-stone-900/30 {flippedCards.has(
      card.id,
    )
      ? 'opacity-0'
      : 'opacity-100'}"
  >
    <div class="flex h-full w-full flex-col items-center justify-center">
      <Callout styling={truncateIfNeeded(card.front)}>
        {card.front}
      </Callout>

      {#if card.mediaUrl}
        <div class="absolute top-2 right-2">
          <img
            src={card.mediaUrl}
            alt="Card media"
            class="h-10 w-10 rounded-md object-cover shadow-sm"
          />
        </div>
      {/if}

      <div class="absolute right-3 bottom-2">
        <Caption1>
          {m.noble_dry_skunk_reside()}
        </Caption1>
      </div>
    </div>
  </div>

  <!-- Card Back -->
  <div
    class="bg-default ring-accent/50 absolute inset-0 flex h-full w-full items-center justify-center rounded-2xl p-5 shadow-sm ring-1 duration-150 backface-hidden hover:shadow-xl {flippedCards.has(
      card.id,
    )
      ? 'opacity-100'
      : 'opacity-0'}"
  >
    <div class="flex h-full w-full flex-col items-center justify-center">
      <Callout styling={truncateIfNeeded(card.back)}>
        {card.back}
      </Callout>
    </div>
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
