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
  import { GraduationCap } from "lucide-svelte";
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
  import message from "$lib/messages.js";
  import NewButton from "$lib/components/UI/forms/buttons/NewButton.svelte";

  let { data } = $props();
  const { students } = data;

  const role = page.params.role;

  $effect(() => {
    const params = new URLSearchParams();

    if ($currentPage > 1) params.set("page", $currentPage.toString());
    if ($pageSize > 0) params.set("per_page", $pageSize.toString());
    if ($searchTerm?.trim()) params.set("search", $searchTerm);
    if ($assigneeStore?.trim()) params.set("assignee", $assigneeStore);

    const queryString = params.toString();
    const newUrl = queryString ? `?${queryString}` : window.location.pathname;

    goto(newUrl, {
      noScroll: true,
      keepFocus: true,
    });
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
            redirect: message.crud.created,
          },
          navigate: true,
        })}
      >
        <NewButton />
      </form>
      {#if data.cards?.length}
        <UniButton
          variant="primary"
          content={m.helpful_slow_flea_catch()}
          Icon={GraduationCap}
          href="flashcards/learn"
        ></UniButton>
      {/if}
    </Merger>
    <SearchBar bind:q={$searchTerm} />
  </VStack>
</Toolbar>

{#if data.decksPaginated.data.length}
  {#if role === "s"}
    <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
      {#each data.decksPaginated.data as deck (deck.id)}
        <DeckCard {deck} />
      {/each}
    </div>
  {:else}
    <Table
      config={deckConfig}
      href="flashcards"
      {students}
      items={data.decksPaginated.data}
    />
  {/if}
{:else}
  <EmptySpace>
    <Title1>{m.noDecks()}</Title1>
  </EmptySpace>
{/if}
