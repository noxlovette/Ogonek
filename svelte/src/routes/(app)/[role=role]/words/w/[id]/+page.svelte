<script lang="ts">
  import { enhance } from "$app/forms";
  import {
    Label,
    WordCard,
    H1,
    UniButton,
    GreySpan,
    HeaderEmbellish,
    EmptySpace,
  } from "$lib/components";
  import { invalidate } from "$app/navigation";

  import { user } from "$lib/stores";
  import { Pencil, UserRoundMinus, UserRoundPlus } from "lucide-svelte";
  import { enhanceForm } from "$lib/utils";
  import Badge from "$lib/components/cards/Badge.svelte";

  let { data } = $props();

  let cards = $derived(data.cards);
  let deck = $derived(data.deck);
  let flippedCards = $state(new Set<string>());

  let isSubscribed = $state(data.isSubscribed);

  const toggleCard = (cardId: string) => {
    flippedCards = new Set(
      flippedCards.has(cardId)
        ? [...flippedCards].filter((id) => id !== cardId)
        : [...flippedCards, cardId],
    );
  };
</script>

<svelte:head>
  <title>{deck.name} | Flashcards</title>
</svelte:head>
<HeaderEmbellish>
  <H1>{deck.name}</H1>

  <div class="flex flex-col gap-2 md:flex-row">
    {#if $user.id === deck.createdBy}
      <UniButton variant="secondary" href="{deck.id}/edit" Icon={Pencil}
        >Edit</UniButton
      >
    {/if}
    <form
      method="POST"
      action="?/subscribe"
      use:enhance={enhanceForm({
        messages: {
          success: isSubscribed ? "Unsubscribed!" : "Subscribed!",
          failure: "Failed to update deck",
          error: "An error occurred",
        },
        handlers: {
          success: async () => {
            isSubscribed = !isSubscribed;
            invalidate("learn:subscribe");
          },
        },
      })}
    >
      <input type="hidden" name="isSubscribed" value={isSubscribed} />
      <UniButton
        Icon={isSubscribed === true ? UserRoundMinus : UserRoundPlus}
        type="submit"
        fullWidth={true}
        variant="primary"
      >
        {isSubscribed ? "Unsubscribe" : "Subscribe"}
      </UniButton>
    </form>
  </div>
</HeaderEmbellish>

<div class="grid gap-8 lg:grid-cols-3">
  <div class="lg:col-span-2">
    <div class="grid gap-4 sm:grid-cols-1 md:grid-cols-2">
      {#each cards as card (card.id)}
        <WordCard bind:flippedCards {card} {toggleCard} />
      {/each}
    </div>

    {#if cards.length === 0}
      <EmptySpace>
        <p class="text-lg text-stone-500 dark:text-stone-400">
          No flashcards available
        </p>
        <p class="mt-2 text-sm text-stone-400 dark:text-stone-500">
          Add some cards by editing this deck
        </p>
      </EmptySpace>
    {/if}
  </div>

  <div class="h-full">
    <div class="sticky top-6">
      <div
        class="bg-default ring-default flex flex-col space-y-3 rounded-lg p-3 shadow-sm"
      >
        <Label>Tags</Label>
        {#if deck.description}
          <div class="flex flex-wrap gap-x-1 gap-y-2">
            {#each deck.description.split(";") as deckTag, index (index)}
              <Badge badgeText={deckTag}></Badge>
            {/each}
          </div>
        {:else}
          <p class="text-stone-500 italic dark:text-stone-400">
            No tags available
          </p>
        {/if}

        <div class="border-t border-stone-100 pt-4 dark:border-stone-700">
          <GreySpan>
            {cards.length}
            {cards.length === 1 ? "card" : "cards"}
          </GreySpan>
        </div>
      </div>
    </div>
  </div>
</div>
