<script lang="ts">
  import type { PageData } from "./$types";
  import {
    H1,
    H2,
    DeckCard,
    Anchor,
    ButtonSubmit,
    Table,
  } from "$lib/components";
  import { enhance } from "$app/forms";
  import { notification } from "$lib/stores";
  import { fade } from "svelte/transition";
  import { page } from "$app/state";

  let { data }: { data: PageData } = $props();
  let { decks } = data;

  const role = page.params.role;
</script>

<svelte:head>
  <title>Flashcards | Review</title>
</svelte:head>

{#if role === "t"}
  <H1>Flashcards</H1>

  <!-- Cards due today section -->
  <div
    class="dark:bg-milk-900 mb-8 rounded-xl bg-white p-4 shadow-sm transition-all hover:shadow-md"
  >
    {#if data.cards?.length}
      <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between">
        <div>
          <H2>{data.cards.length} cards due today</H2>
          <p class="text-milk-600 dark:text-milk-400 mt-1">
            Time for some learning!
          </p>
        </div>
        <Anchor href="words/learn">Start Review →</Anchor>
      </div>
    {:else}
      <div class="py-8 text-center" in:fade={{ duration: 300 }}>
        <div
          class="bg-cacao-50 dark:bg-milk-800 mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-full"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            class="text-cacao-500 dark:text-milk-300 h-8 w-8"
            viewBox="0 0 20 20"
            fill="currentColor"
          >
            <path
              fill-rule="evenodd"
              d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
              clip-rule="evenodd"
            />
          </svg>
        </div>
        <H2>All Caught Up! 🎉</H2>
        <p class="text-milk-600 dark:text-milk-400 mt-2">
          Check back later for new cards to review
        </p>
      </div>
    {/if}
  </div>

  <!-- Decks Management Section -->
  <div>
    <div
      class="mb-6 flex flex-col sm:flex-row sm:items-center sm:justify-between"
    >
      <H2>Your Decks</H2>

      <form
        method="POST"
        class="mt-3 sm:mt-0"
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

    {#if decks.length}
      <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
        {#each decks as deck}
          <DeckCard {deck} />
        {/each}
      </div>
    {:else}
      <div class="bg-milk-50 dark:bg-milk-800 rounded-xl py-12 text-center">
        <div
          class="bg-milk-100 dark:bg-milk-700 mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-full"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            class="text-milk-500 dark:text-milk-400 h-8 w-8"
            viewBox="0 0 20 20"
            fill="currentColor"
          >
            <path
              d="M9 4.804A7.968 7.968 0 005.5 4c-1.255 0-2.443.29-3.5.804v10A7.969 7.969 0 015.5 14c1.669 0 3.218.51 4.5 1.385A7.962 7.962 0 0114.5 14c1.255 0 2.443.29 3.5.804v-10A7.968 7.968 0 0014.5 4c-1.255 0-2.443.29-3.5.804V12a1 1 0 11-2 0V4.804z"
            />
          </svg>
        </div>
        <p class="text-milk-800 dark:text-milk-300 mb-2 text-lg font-medium">
          No decks yet
        </p>
        <p class="text-milk-600 dark:text-milk-400 mx-auto mb-6 max-w-md">
          Create your first deck to start learning!
        </p>
      </div>
    {/if}
  </div>
{:else}
  <Table></Table>
{/if}
