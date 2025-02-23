<script lang="ts">
  import type { PageData } from "./$types";
  import { H1, H2, DeckCard, Anchor, ButtonSubmit } from "$lib/components";
  import { enhance } from "$app/forms";
  import { notification } from "$lib/stores";

  let { data }: { data: PageData } = $props();
  let { decks } = data;
</script>

<svelte:head>
  <title>Flashcards | Review</title>
</svelte:head>

<H1>Your Flashcards</H1>
<div class="rounded-xl bg-white p-6 shadow-sm">
  {#if data.cards?.length}
    <div class="flex items-center justify-between">
      <div>
        <H2>{data.cards.length} cards due today</H2>
        <p class="text-milk-600 mt-1">Time for some learning!</p>
      </div>

      <Anchor href="words/learn">Start Review →</Anchor>
    </div>
  {:else}
    <div class="py-8 text-center">
      <H2>All Caught Up! 🎉</H2>
      <p class="text-milk-600 mt-2">Check back later for new cards to review</p>
    </div>
  {/if}
</div>

<!-- Decks Management Section -->
<div>
  <div class="mb-4 flex items-center justify-between">
    <H2>Your Decks</H2>
    <form
      method="POST"
      use:enhance={() => {
        return async ({ result, update }) => {
          if (result.type === "redirect") {
            notification.set({
              message: "Created New Deck",
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
      <ButtonSubmit buttonName="New Deck +"></ButtonSubmit>
    </form>
  </div>

  <div class="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3">
    {#each decks as deck}
      <DeckCard {deck} />
    {/each}

    {#if !decks.length}
      <div class="bg-milk-50 col-span-full rounded-xl py-8 text-center">
        <p class="text-milk-600">Create your first deck to start learning!</p>
      </div>
    {/if}
  </div>
</div>
