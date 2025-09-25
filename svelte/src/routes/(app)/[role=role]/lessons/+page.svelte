<script lang="ts">
  import {
    LargeTitle,
    Table,
    LessonCard,
    UniButton,
    Toolbar,
    EmptySpace,
    Title3,
    SearchBar,
    TableSkeleton,
    Divider,
    Merger,
    Title1,
  } from "$lib/components";
  import { enhance } from "$app/forms";
  import { enhanceForm } from "$lib/utils";
  import { page } from "$app/state";
  import type { TableConfig, LessonSmall } from "$lib/types/index.js";
  import { formatDate } from "$lib/utils";

  import {
    user,
    searchTerm,
    pageSize,
    currentPage,
    assigneeStore,
  } from "$lib/stores";
  import { goto } from "$app/navigation";
  import { Plus } from "lucide-svelte";
  import { m } from "$lib/paraglide/messages.js";
  import LoadingCard from "$lib/components/cards/LoadingCard.svelte";
  import VStack from "$lib/components/UI/layout/VStack.svelte";
  import NewButton from "$lib/components/UI/forms/buttons/NewButton.svelte";

  let { data } = $props();

  let role = page.params.role;
  let href = role === "t" ? "/t/lessons" : `/s/lessons`;

  const lessonConfig: TableConfig<LessonSmall> = {
    columns: [
      { key: "title", label: m.title() },
      { key: "topic", label: m.topic() },
      {
        key: "assigneeName",
        label: m.assignee(),
        formatter: (value: string | boolean | undefined | null) =>
          value === $user.name ? m.notAssigned() : String(value),
      },
      {
        key: "createdAt",
        label: m.created(),
        formatter: (value: string | boolean | undefined | null) =>
          formatDate(String(value)),
      },
    ],
  };

  $effect(() => {
    const params = new URLSearchParams();

    if ($searchTerm?.trim()) params.set("search", $searchTerm);
    if ($pageSize > 0) params.set("per_page", $pageSize.toString());
    if ($currentPage > 1) params.set("page", $currentPage.toString());
    if ($assigneeStore?.trim()) params.set("assignee", $assigneeStore);

    const queryString = params.toString();
    const newUrl = queryString ? `?${queryString}` : window.location.pathname;

    goto(newUrl, {
      noScroll: true,
      keepFocus: true,
    });
  });
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

{#await data.lessonsPaginated}
  {#if role === "s"}
    <div class="grid grid-cols-1 gap-4 md:grid-cols-3">
      <LoadingCard />
      <LoadingCard />
      <LoadingCard />
    </div>
  {:else}
    <TableSkeleton />
  {/if}
{:then lessons}
  {#if lessons.data.length < 1}
    <EmptySpace>
      <Title1>{m.noLessons()}</Title1>
    </EmptySpace>
  {/if}
  {#if role === "s"}
    <div class="grid grid-cols-1 gap-4 md:grid-cols-3">
      {#each lessons.data as lesson (lesson.id)}
        <LessonCard {lesson} />
      {/each}
    </div>
  {:else}
    <Table items={lessons.data} {href} config={lessonConfig} />
  {/if}
{:catch error: App.Error}
  <p>Error loading lessons: {error.errorID}</p>
{/await}

<svelte:head>
  <title>{m.lessons()}</title>
</svelte:head>
