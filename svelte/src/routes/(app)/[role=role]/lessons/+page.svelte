<script lang="ts">
  import {
    LargeTitle,
    Table,
    LessonCard,
    Toolbar,
    Paginator,
    EmptySpace,
    SearchBar,
    Divider,
    Merger,
    Title1,
    Headline,
    HStack,
    TickMorph,
    Subheadline,
    NewButton,
    TableRow,
    TableHead,
    TableBody,
    TableFooter,
    SortDate,
    Caption1,
    VStack,
    UniButton,
  } from "$lib/components";
  import { enhance } from "$app/forms";
  import { enhanceForm, formatDateOnly } from "$lib/utils";
  import { page as sveltePage } from "$app/state";

  import {
    searchTerm,
    pageSize,
    currentPage,
    assigneeStore,
    sortBy,
    sortOrder,
  } from "$lib/stores";
  import { goto } from "$app/navigation";
  import { m } from "$lib/paraglide/messages.js";
  import { ChevronLeft, ChevronRight } from "@lucide/svelte";

  let { data } = $props();

  const { page, totalPages, count, perPage } = data.lessonsPaginated;
  let role = sveltePage.params.role;

  const lessons = $derived(data.lessonsPaginated.data);

  $effect(() => {
    const params = new URLSearchParams();

    if ($searchTerm?.trim()) params.set("search", $searchTerm);
    if ($pageSize > 0) params.set("per_page", $pageSize.toString());
    if ($currentPage > 1) params.set("page", $currentPage.toString());
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

  let selected: string[] = $state([]);
</script>

<Toolbar>
  <LargeTitle>{m.lessons()}</LargeTitle>
  <Divider />

  <VStack>
    {#if role === "t"}
      <Merger>
        <form
          action="?/new"
          method="post"
          use:enhance={enhanceForm({
            messages: {
              redirect: m.created(),
            },
            navigate: true,
          })}
        >
          <NewButton />
        </form>
      </Merger>
    {/if}
    <SearchBar bind:q={$searchTerm} />
  </VStack>
</Toolbar>

{#if lessons.length < 1}
  <EmptySpace>
    <Title1>{m.noLessons()}</Title1>
  </EmptySpace>
{/if}
{#if role === "s"}
  <div class="grid grid-cols-1 gap-4 md:grid-cols-3">
    {#each lessons as lesson (lesson.id)}
      <LessonCard {lesson} />
    {/each}
  </div>
{:else}
  <Table>
    <TableHead>
      <TickMorph
        noText={true}
        bind:group={selected}
        value={lessons.map((lesson) => lesson.id)}
      />
      {#if selected.length >= 1}
        <Subheadline>
          Выбрано {selected.length} из {lessons.length}
        </Subheadline>
      {/if}
      <Divider />

      <SortDate />
    </TableHead>
    <TableBody>
      {#each lessons as lesson (lesson.id)}
        <div class="bg-clickable flex items-center px-2">
          <TickMorph noText={true} bind:group={selected} value={lesson.id} />
          <TableRow href={`/${sveltePage.params.role}/lessons/${lesson.id}`}>
            <HStack override="gap-1 items-start">
              <Headline>
                {lesson.title}
              </Headline>
              <Caption1>
                {lesson.assigneeName}
              </Caption1>
            </HStack>
            <Divider />
            <Caption1>
              {formatDateOnly(lesson.createdAt)}
            </Caption1>
          </TableRow>
        </div>
      {/each}
    </TableBody>
    <TableFooter>
      <Paginator {page} {count} {perPage} {totalPages}></Paginator>
    </TableFooter>
  </Table>
{/if}

<svelte:head>
  <title>{m.lessons()}</title>
</svelte:head>
