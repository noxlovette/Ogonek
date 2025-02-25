<script lang="ts">
  import { enhance } from "$app/forms";
  import { MetaData, ButtonEdit, Label, WordCard } from "$lib/components";
  import H1 from "$lib/components/typography/H1.svelte";
  import ButtonSubmit from "$lib/components/UI/buttons/ButtonSubmit.svelte";
  import { notification } from "$lib/stores";
  let isSubmitting = $state(true);

  let { data } = $props();
  let { deck, cards } = data;
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
<H1>{deck.name}</H1>

{#if deck.shared}
  <span
    class="rounded-full bg-stone-100 px-2 py-1 text-xs font-medium text-stone-700 dark:bg-stone-700 dark:text-stone-100"
  >
    Shared
  </span>
{/if}

<div class="grid w-full grid-cols-1 gap-8 lg:grid-cols-3">
  <div class="col-span-2 space-y-6 rounded-lg">
    <div class="flex items-center justify-between">
      <h2 class="text-xl font-semibold">Flashcards</h2>
      <span class="text-sm text-stone-500">{cards.length} cards</span>
    </div>

    <div class="grid grid-cols-1 gap-4 md:grid-cols-2">
      {#each cards as card (card.id)}
        <WordCard bind:flippedCards {card} {toggleCard} />
      {/each}
    </div>
  </div>

  <div class="col-span-2 flex h-full flex-col space-y-4 md:col-span-1">
    <div class="rounded-lg bg-white p-4 shadow-sm dark:bg-stone-800">
      <Label>Description</Label>
      <p class="mt-2 text-lg">
        {deck.description || "No description available"}
      </p>
    </div>

    <div class="rounded-lg bg-white p-4 shadow-sm dark:bg-stone-800">
      <Label>Progress</Label>
      <div class="mt-4 space-y-2">
        <div class="flex justify-between">
          <span>Due today</span>
          <span class="text-cacao-600 dark:text-cacao-400 font-medium">
            {data.dueToday ?? 0}
          </span>
        </div>
        <div class="flex justify-between">
          <span>Total reviewed</span>
          <span class="font-medium">
            {data.totalReviewed ?? 0}
          </span>
        </div>
      </div>
    </div>

    <div class="flex gap-2">
      <ButtonEdit href="{deck.id}/edit" />
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
        <ButtonSubmit buttonName="Add to Routine"></ButtonSubmit>
      </form>
    </div>
  </div>
</div>
