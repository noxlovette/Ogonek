<script lang="ts">
  import type { PageData } from "./$types";
  import {
    H1,
    H2,
    H3,
    DeckCard,
    Table,
    UniButton,
    HeaderEmbellish,
  } from "$lib/components";
  import { enhance } from "$app/forms";
  import { enhanceForm } from "$lib/utils";
  import { fade } from "svelte/transition";
  import { page } from "$app/state";
  import type { TableConfig, Deck } from "$lib/types";
  import {
    ArrowBigRight,
    BookOpen,
    Cat,
    CheckCheck,
    PlusCircle,
    Ticket,
  } from "lucide-svelte";
  import {
    searchTerm,
    pageSize,
    currentPage,
    assigneeStore,
  } from "$lib/stores";
  import { goto } from "$app/navigation";
  import { formatDate } from "@noxlovette/svarog";

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
          formatDate(String(value)),
      },
    ],
  };
</script>

<svelte:head>
  <title>Flashcards | Review</title>
</svelte:head>

<HeaderEmbellish>
  <div>
    <H1>Flashcards</H1>
    <H3>
      {#if data.cards?.length}
        {data.cards.length} cards due today
      {:else}
        All Caught Up! 🎉
      {/if}
    </H3>
  </div>
  <div>
    {#if data.cards?.length}
      <UniButton
        variant="primary"
        Icon={ArrowBigRight}
        iconPosition="right"
        href="words/learn">Start Review</UniButton
      >
    {:else}
      <div
        class="mx-auto flex h-16 w-16 items-center justify-center rounded-full bg-stone-200 dark:bg-stone-800"
      >
        <Cat />
      </div>
    {/if}
  </div>
</HeaderEmbellish>

{#if role === "s"}
  <div class="space-y-4">
    <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between">
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
      <div
        class="rounded-lg bg-stone-50 py-12 text-center shadow-sm ring ring-stone-200 dark:bg-stone-800 dark:ring-stone-900"
      >
        <div
          class="mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-full bg-stone-200 dark:bg-stone-700"
        >
          <BookOpen />
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
