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
    TickMorph,
    Subheadline,
    SortDate,
    DeleteButton,
    TableBody,
    TableRow,
    HStack,
    Headline,
    Caption1,
    TableFooter,
    Paginator,
    Badge,
  } from "$lib/components";
  import { enhance } from "$app/forms";
  import { enhanceForm } from "$lib/utils";
  import { page as sveltePage } from "$app/state";
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

  const role = sveltePage.params.role;
  const { page, totalPages, count, perPage } = $derived(data.decksPaginated);
  const decks = $derived(data.decksPaginated.data);
  let selected: string[] = $state([]);
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
    <Table bind:selected>
      {#each selected as id}
        <input type="hidden" name="toDelete" value={id} />
      {/each}
      <TableHead>
        <TickMorph
          noText={true}
          bind:group={selected}
          value={decks.map((deck) => deck.id)}
        />
        {#if selected.length >= 1}
          <Subheadline>
            Выбрано {selected.length} из {decks.length}
          </Subheadline>
        {:else}
          <Subheadline>Выбрать все</Subheadline>
        {/if}
        <Divider />

        {#if selected.length == 0}
          <SortDate />
        {:else}
          <Merger>
            <DeleteButton />
          </Merger>
        {/if}
      </TableHead>
      <TableBody>
        {#each decks as deck (deck.id)}
          <div class="bg-clickable flex items-center px-2">
            <TickMorph noText={true} bind:group={selected} value={deck.id} />
            <TableRow href={`/${sveltePage.params.role}/decks/${deck.id}`}>
              <HStack override="gap-1 items-start">
                <Headline>
                  {deck.title}
                </Headline>
                <Caption1>
                  {deck.assigneeName}
                </Caption1>
              </HStack>
              <Divider />
              <Badge>
                {deck.cardCount} карточек
              </Badge>
              <Badge urgency={deck.isSubscribed ? "green" : "normal"}>
                {deck.isSubscribed ? "Подписаны" : "Не подписаны"}
              </Badge>
            </TableRow>
          </div>
        {/each}
      </TableBody>
      <TableFooter>
        <Paginator {page} {count} {perPage} {totalPages}></Paginator>
      </TableFooter>
    </Table>
  {/if}
{:else}
  <EmptySpace>
    <Title1>{m.noDecks()}</Title1>
  </EmptySpace>
{/if}
