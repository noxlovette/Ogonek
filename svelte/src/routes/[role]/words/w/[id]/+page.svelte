<script lang="ts">
  import { enhance } from "$app/forms";
  import {
    MetaData,
    ButtonEdit,
    Label,
    WordCard,
    H1,
    ButtonSubmit,
  } from "$lib/components";

  import { notification, user } from "$lib/stores";

  let isSubmitting = $state(false);
  let { data } = $props();

  let cards = $derived(data.cards);
  let deck = $derived(data.deck);
  let flippedCards = $state(new Set<string>());

  const toggleCard = (cardId: string) => {
    flippedCards = new Set(
      flippedCards.has(cardId)
        ? [...flippedCards].filter((id) => id !== cardId)
        : [...flippedCards, cardId],
    );
  };
</script>

<MetaData title="{deck.name} | Flashcards" robots="noindex, nofollow" />

<div class="mb-6 flex flex-wrap items-center justify-between gap-4">
  <H1>{deck.name}</H1>
  <div class="flex gap-2">
    {#if $user.sub === deck.created_by}
      <ButtonEdit href="{deck.id}/edit" />
    {/if}
    <form
      method="POST"
      use:enhance={() => {
        isSubmitting = true;
        return async ({ result, update }) => {
          isSubmitting = false;
          if (result.type === "success") {
            notification.set({
              message: "Added to Routine",
              type: "success",
            });
            update();
          } else if (result.type === "failure") {
            notification.set({
              message: String(result.data?.message),
              type: "error",
            });
          }
        };
      }}
    >
      <ButtonSubmit buttonName="Add to Routine" bind:isSubmitting
      ></ButtonSubmit>
    </form>
  </div>
</div>

<div class="grid gap-8 lg:grid-cols-3">
  <!-- Main content area - Flashcards -->
  <div class="space-y-6 lg:col-span-2">
    <div
      class="dark:bg-milk-900 flex items-center justify-between rounded-lg bg-white p-4 shadow-sm"
    >
      <h2 class="text-xl font-semibold">Flashcards</h2>
      <span
        class="bg-milk-100 dark:bg-milk-700 text-milk-600 dark:text-milk-300 rounded-full px-3 py-1 text-sm font-medium"
      >
        {cards.length}
        {cards.length === 1 ? "card" : "cards"}
      </span>
    </div>

    <div class="grid gap-4 sm:grid-cols-1 md:grid-cols-2">
      {#each cards as card (card.id)}
        <WordCard bind:flippedCards {card} {toggleCard} />
      {/each}
    </div>

    {#if cards.length === 0}
      <div
        class="border-milk-200 dark:border-milk-700 flex h-40 flex-col items-center justify-center rounded-lg border-2 border-dashed p-6 text-center"
      >
        <p class="text-milk-500 dark:text-milk-400 text-lg">
          No flashcards available
        </p>
        <p class="text-milk-400 dark:text-milk-500 mt-2 text-sm">
          Add some cards by editing this deck
        </p>
      </div>
    {/if}
  </div>

  <!-- Sidebar - Deck Info -->
  <div class="flex flex-col space-y-4">
    <div class="dark:bg-milk-900 rounded-lg bg-white p-6 shadow-sm">
      <Label>Description</Label>
      {#if deck.description}
        <p class="mt-2 text-lg">{deck.description}</p>
      {:else}
        <p class="text-milk-500 dark:text-milk-400 mt-2 italic">
          No description available
        </p>
      {/if}

      {#if deck.visibility}
        <div class="border-milk-100 dark:border-milk-700 mt-4 border-t pt-4">
          <Label>Visibility</Label>
          <div class="mt-2 flex items-center gap-2">
            <span
              class="bg-milk-100 dark:bg-milk-700 text-milk-600 dark:text-milk-300 rounded-full px-3 py-1 text-sm font-medium capitalize"
            >
              {deck.visibility}
            </span>
          </div>
        </div>
      {/if}
    </div>
  </div>
</div>
