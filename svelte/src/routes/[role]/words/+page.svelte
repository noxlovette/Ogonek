<script lang="ts">
  import type { PageData } from "./$types";
  import { H1, H2, DeckCard, Table, UniButton } from "$lib/components";
  import { enhance } from "$app/forms";
  import { enhanceForm } from "$lib/utils";
  import { fade } from "svelte/transition";
  import { page } from "$app/state";
  import type { TableConfig, Deck } from "$lib/types";
  import { formatDateTime } from "$lib/utils";
  import { ArrowBigRight, PlusCircle } from "lucide-svelte";
  import {
    searchTerm,
    pageSize,
    currentPage,
    assigneeStore,
  } from "$lib/stores";
  import { goto } from "$app/navigation";

  let { data }: { data: PageData } = $props();
  let { decks, students } = $derived(data);

  const role = page.params.role;

  $effect(() => {
    goto(
      `?search=${$searchTerm}&page_size=${$pageSize}&page=${$currentPage}&assignee=${$assigneeStore}`,
      {
        noScroll: true,
        keepFocus: true,
      },
    );
  });

  const deckConfig: TableConfig<Deck> = {
    columns: [
      { key: "name", label: "Deck Name" },
      {
        key: "description",
        label: "Description",
        formatter: (value: string | boolean | undefined) =>
          value
            ? String(value).substring(0, 50) +
              (String(value).length > 50 ? "..." : "")
            : "No description",
      },
      {
        key: "visibility",
        label: "Visibility",
        formatter: (value: string | boolean | undefined) => {
          switch (String(value)) {
            case "public":
              return "Public 🌐";
            case "private":
              return "Private 🔒";
            case "assigned":
              return "Assigned 👤";
            default:
              return String(value);
          }
        },
      },
      {
        key: "createdAt",
        label: "Created",
        formatter: (value: string | boolean | undefined) =>
          formatDateTime(String(value)),
      },
    ],
  };
</script>

<svelte:head>
  <title>Flashcards | Review</title>
</svelte:head>

<H1>Flashcards</H1>

<!-- Cards due today section -->
<div
  class="mb-8 rounded-xl bg-white p-4 shadow-sm transition-all hover:shadow-md dark:bg-stone-900"
>
  {#if data.cards?.length}
    <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between">
      <div>
        <H2>{data.cards.length} cards due today</H2>
        <p class="mt-1 text-stone-600 dark:text-stone-400">
          Time for some learning!
        </p>
      </div>

      <UniButton
        variant="primary"
        Icon={ArrowBigRight}
        iconPosition="right"
        href="words/learn">Start Review</UniButton
      >
    </div>
  {:else}
    <div class="py-8 text-center" in:fade={{ duration: 300 }}>
      <div
        class="bg-cacao-50 mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-full dark:bg-stone-800"
      >
        <svg
          xmlns="http://www.w3.org/2000/svg"
          class="text-cacao-500 h-8 w-8 dark:text-stone-300"
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
      <p class="mt-2 text-stone-600 dark:text-stone-400">
        Check back later for new cards to review
      </p>
    </div>
  {/if}
</div>
{#if role === "s"}
  <!-- Decks Management Section -->
  <div>
    <div
      class="mb-6 flex flex-col sm:flex-row sm:items-center sm:justify-between"
    >
      <H2>Your Decks</H2>

      <form
        method="POST"
        class="mt-3 sm:mt-0"
        action="?/new"
        use:enhance={enhanceForm({
          messages: {
            redirect: "New Deck Created",
            defaultError: "Something's off",
          },
          navigate: true,
        })}
      >
        <UniButton
          Icon={PlusCircle}
          type="submit"
          variant="outline"
          iconPosition="right">New Deck</UniButton
        >
      </form>
    </div>

    {#if decks.length}
      <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
        {#each decks as deck}
          <DeckCard {deck} />
        {/each}
      </div>
    {:else}
      <div class="rounded-xl bg-stone-50 py-12 text-center dark:bg-stone-800">
        <div
          class="mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-full bg-stone-100 dark:bg-stone-700"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            class="h-8 w-8 text-stone-500 dark:text-stone-400"
            viewBox="0 0 20 20"
            fill="currentColor"
          >
            <path
              d="M9 4.804A7.968 7.968 0 005.5 4c-1.255 0-2.443.29-3.5.804v10A7.969 7.969 0 015.5 14c1.669 0 3.218.51 4.5 1.385A7.962 7.962 0 0114.5 14c1.255 0 2.443.29 3.5.804v-10A7.968 7.968 0 0014.5 4c-1.255 0-2.443.29-3.5.804V12a1 1 0 11-2 0V4.804z"
            />
          </svg>
        </div>
        <p class="mb-2 text-lg font-medium text-stone-800 dark:text-stone-300">
          No decks yet
        </p>
        <p class="mx-auto mb-6 max-w-md text-stone-600 dark:text-stone-400">
          Create your first deck to start learning!
        </p>
      </div>
    {/if}
  </div>
{:else}
  <Table
    config={deckConfig}
    href="words/w"
    {students}
    items={decks}
    total={decks.length}
  />
{/if}
