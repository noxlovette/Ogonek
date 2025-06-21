<script lang="ts">
  import type { PageData } from "./$types";
  import {
    H1,
    H3,
    DeckCard,
    Table,
    UniButton,
    HeaderEmbellish,
  } from "$lib/components";
  import { enhance } from "$app/forms";
  import { enhanceForm } from "$lib/utils";
  import { page } from "$app/state";
  import type { TableConfig, Deck } from "$lib/types";
  import { ArrowBigRight, PlusCircle, ShoppingBag } from "lucide-svelte";
  import {
    searchTerm,
    pageSize,
    currentPage,
    assigneeStore,
  } from "$lib/stores";
  import { goto } from "$app/navigation";
  import { formatDate } from "@noxlovette/svarog";
  import EmptySpace from "$lib/components/typography/EmptySpace.svelte";

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
  <div class="flex flex-col gap-3 md:flex-row md:items-baseline md:gap-4">
    <H1>Flashcards</H1>
    <H3>
      {#if data.cards?.length}
        {data.cards.length} cards due today
      {:else}
        All Caught Up!
      {/if}
    </H3>
  </div>

  <div class="flex flex-col gap-3 md:flex-row md:gap-4">
    <form
      action="?/new"
      method="post"
      use:enhance={enhanceForm({
        messages: {
          redirect: "New Deck Created",
        },
        navigate: true,
      })}
    >
      <UniButton
        fullWidth={true}
        Icon={PlusCircle}
        type="submit"
        variant="primary">New</UniButton
      >
    </form>
    <UniButton
      Icon={ShoppingBag}
      variant="primary"
      href="words/marketplace"
      styling="hidden md:flex">Marketplace</UniButton
    >
    {#if data.cards?.length}
      <UniButton variant="primary" Icon={ArrowBigRight} href="words/learn"
        >Start Review</UniButton
      >
    {/if}
  </div>
</HeaderEmbellish>
{#if role === "s"}
  <div class="space-y-4">
    {#if decks.length}
      <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
        {#each decks as deck (deck.id)}
          <DeckCard {deck} />
        {/each}
      </div>
    {:else}
      <EmptySpace>
        <H3>No decks</H3>
      </EmptySpace>
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
