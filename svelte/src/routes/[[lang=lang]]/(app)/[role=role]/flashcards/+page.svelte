<script lang="ts">
  import {
    LargeTitle,
    DeckCard,
    Table,
    UniButton,
    Toolbar,
    LoadingCard,
    SearchBar,
    Divider,
    Merger,
    VStack,
  } from "$lib/components";
  import { enhance } from "$app/forms";
  import { enhanceForm } from "$lib/utils";
  import { page } from "$app/state";
  import type { TableConfig, DeckSmall } from "$lib/types/index.js";
  import { GraduationCap, Plus } from "lucide-svelte";
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
  import Title1 from "$lib/components/typography/Title1.svelte";

  let { data } = $props();
  const { students } = data;

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
      { key: "title", label: m.title() },
      {
        key: "description",
        label: m.equal_key_gazelle_attend(),
        formatter: (value: string | boolean | undefined | null | number) =>
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
        formatter: (value: string | boolean | undefined | null | number) =>
          value === true ? "◉" : "○",
      },
    ],
  };
</script>

<svelte:head>
  <title>{m.flashcards()}</title>
</svelte:head>

<Toolbar>
  <LargeTitle>{m.flashcards()}</LargeTitle>
  <Divider />
  <VStack>
    <Merger>
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
        <UniButton Icon={Plus} type="submit" variant="primary"
          >{m.new()}</UniButton
        >
      </form>
      {#if data.cards?.length}
        <UniButton
          variant="primary"
          Icon={GraduationCap}
          href="flashcards/learn">{m.helpful_slow_flea_catch()}</UniButton
        >
      {/if}
    </Merger>
    <SearchBar />
  </VStack>
</Toolbar>

{#await data.decksResponse}
  {#if role === "s"}
    <div class="grid grid-cols-1 gap-4 md:grid-cols-3">
      <LoadingCard />
      <LoadingCard />
      <LoadingCard />
    </div>
  {:else}
    <TableSkeleton />
  {/if}
{:then decks}
  {#if role === "s"}
    {#if decks.data.length}
      <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
        {#each decks.data as deck (deck.id)}
          <DeckCard {deck} />
        {/each}
      </div>
    {:else}
      <EmptySpace>
        <Title1>{m.noDecks()}</Title1>
      </EmptySpace>
    {/if}
  {:else}
    <Table
      config={deckConfig}
      href="flashcards"
      {students}
      items={decks.data}
      total={decks.data.length}
    />
  {/if}
{:catch error: App.Error}
  <p>Error loading decks: {error.errorID}</p>
{/await}
