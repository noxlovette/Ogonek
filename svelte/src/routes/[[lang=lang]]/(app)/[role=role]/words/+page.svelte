<script lang="ts">
  import {
    H1,
    H3,
    DeckCard,
    Table,
    UniButton,
    HeaderEmbellish,
    LoadingCard,
    SearchBar,
  } from "$lib/components";
  import { enhance } from "$app/forms";
  import { enhanceForm } from "$lib/utils";
  import { page } from "$app/state";
  import type { TableConfig, DeckSmall } from "$lib/types";
  import { ArrowBigRight, PlusCircle, ShoppingBag } from "lucide-svelte";
  import { m } from "$lib/paraglide/messages";
  import {
    searchTerm,
    pageSize,
    currentPage,
    assigneeStore,
  } from "$lib/stores";
  import { goto } from "$app/navigation";
  import EmptySpace from "$lib/components/typography/EmptySpace.svelte";
  import TableSkeleton from "$lib/components/UI/interactive/TableSkeleton.svelte";

  let { data } = $props();
  let { students } = $derived(data);

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

  const deckConfig: TableConfig<DeckSmall> = {
    columns: [
      { key: "name", label: m.title() },
      {
        key: "description",
        label: m.equal_key_gazelle_attend(),
        formatter: (value: string | boolean | undefined) =>
          value
            ? String(value).substring(0, 50) +
              (String(value).length > 50 ? "..." : "")
            : m.simple_east_crocodile_spark(),
      },
      {
        key: "assigneeName",
        label: m.assignee(),
      },
      {
        key: "isSubscribed",
        label: m.stout_royal_macaw_fear(),
        formatter: (value: string | boolean | undefined) =>
          value === true ? "◉" : "○",
      },
    ],
  };
</script>

<svelte:head>
  <title>Flashcards | Review</title>
</svelte:head>

<HeaderEmbellish>
  <div class="flex flex-col gap-3 md:flex-row md:items-baseline md:gap-4">
    <H1>{m.flashcards()}</H1>
    <H3>
      {#if data.cards?.length}
        {data.cards.length} {m.blue_solid_wren_feel()}
      {:else}
        {m.livid_trite_squirrel_cuddle()}
      {/if}
    </H3>
  </div>

  <div class="flex flex-col gap-3 md:flex-row md:gap-4">
    <form
      action="?/new"
      method="post"
      use:enhance={enhanceForm({
        messages: {
          redirect: m.newDeckCreated(),
        },
        navigate: true,
      })}
    >
      <UniButton
        fullWidth={true}
        Icon={PlusCircle}
        type="submit"
        variant="primary">{m.new()}</UniButton
      >
    </form>
    <UniButton
      Icon={ShoppingBag}
      variant="primary"
      href="words/marketplace"
      styling="hidden md:flex">{m.marketplace()}</UniButton
    >
    {#if data.cards?.length}
      <UniButton variant="primary" Icon={ArrowBigRight} href="words/learn"
        >{m.helpful_slow_flea_catch()}</UniButton
      >
    {/if}
  </div>
</HeaderEmbellish>

<SearchBar />
{#await data.decksResponse}
  {#if role === "s"}
    <div class="grid grid-cols-1 gap-4 md:grid-cols-3">
      {#each Array(6) as index (index)}
        <LoadingCard />
      {/each}
    </div>
  {:else}
    <TableSkeleton />
  {/if}
{:then decks}
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
          <H3>{m.noDecks()}</H3>
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
{/await}
