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

  import { notification, user } from "$lib/stores";
  import { Pencil, Share, UserRoundMinus, UserRoundPlus } from "lucide-svelte";
  import { enhanceForm } from "$lib/utils";

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
    <form
      class="w-full"
      method="POST"
      action="?/share"
      use:enhance={enhanceForm({
        messages: {
          failure: "Failed to generate link",
        },
        handlers: {
          success: async (result) => {
            const link = String(result.data?.link);
            try {
              await navigator.clipboard.writeText(link);
              notification.set({
                message: "Link copied to clipboard!",
                type: "success",
              });
            } catch (e) {
              console.error(e);
              notification.set({
                message: "Failed to copy link",
                type: "error",
              });
            }
          },
        },
      })}
    >
      <UniButton Icon={Share} fullWidth={true} type="submit" variant="outline">
        Share Deck
      </UniButton>
    </form>
    {#if $user.id === deck.createdBy}
      <UniButton variant="outline" href="{deck.id}/edit" Icon={Pencil}
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
        variant="outline"
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
        class="space-y-3 rounded-lg bg-white p-4 shadow-sm ring-1 ring-stone-300/40 dark:bg-stone-800"
      >
        <Label>Description</Label>
        {#if deck.description}
          <p class="text-lg">{deck.description}</p>
        {:else}
          <p class="text-stone-500 italic dark:text-stone-400">
            No description available
          </p>
        {/if}

        {#if deck.visibility}
          <div class="border-t border-stone-100 pt-4 dark:border-stone-700">
            <Label>Info</Label>
            <div class="flex items-center gap-2">
              <GreySpan>
                {deck.visibility}
              </GreySpan>
              <GreySpan>
                {cards.length}
                {cards.length === 1 ? "card" : "cards"}
              </GreySpan>
            </div>
          </div>
        {/if}
      </div>
    </div>
  </div>
</div>
