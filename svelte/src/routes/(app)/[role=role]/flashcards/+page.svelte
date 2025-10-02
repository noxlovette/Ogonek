<script lang="ts">
  import {
    LargeTitle,
    DeckCard,
    Table,
    UniButton,
    Toolbar,
    SearchBar,
    Divider,
    Merger,
    VStack,
    EmptySpace,
    Title1,
    NewButton,
    TableHead,
  } from "$lib/components";
  import { enhance } from "$app/forms";
  import { enhanceForm } from "$lib/utils";
  import { page } from "$app/state";
  import { GraduationCap } from "@lucide/svelte";
  import { m } from "$lib/paraglide/messages";
  import {
    searchTerm,
    pageSize,
    currentPage,
    assigneeStore,
    sortBy,
    sortOrder,
  } from "$lib/stores";
  import { goto } from "$app/navigation";
  import message from "$lib/messages.js";

  let { data } = $props();

  const role = page.params.role;

  $effect(() => {
    const params = new URLSearchParams();

    if ($currentPage > 1) params.set("page", $currentPage.toString());
    if ($pageSize > 0) params.set("per_page", $pageSize.toString());
    if ($searchTerm?.trim()) params.set("search", $searchTerm);
    if ($assigneeStore?.trim()) params.set("assignee", $assigneeStore);
    if ($sortBy?.trim()) params.set("sort_by", $sortBy);
    if ($sortOrder?.trim()) params.set("sort_order", $sortOrder);

    const queryString = params.toString();
    const newUrl = queryString ? `?${queryString}` : window.location.pathname;

    goto(newUrl, {
      noScroll: true,
      keepFocus: true,
    });
  });
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
          fill={false}
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
    <Table>
      <TableHead>Hello</TableHead>
    </Table>
  {/if}
{:else}
  <EmptySpace>
    <Title1>{m.noDecks()}</Title1>
  </EmptySpace>
{/if}
