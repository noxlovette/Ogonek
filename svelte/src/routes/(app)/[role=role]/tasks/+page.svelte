<script lang="ts">
  import {
    LargeTitle,
    Divider,
    Merger,
    Table,
    UniButton,
    Toolbar,
    SearchBar,
    TaskCard,
    EmptySpace,
    VStack,
    Title1,
    NewButton,
    TableHead,
    Caption1,
    TableRow,
    TableBody,
    TableFooter,
    Paginator,
    DeleteButton,
    SortDate,
    Subheadline,
    TickMorph,
    HStack,
    Headline,
  } from "$lib/components";

  import { enhance } from "$app/forms";
  import { page as sveltePage } from "$app/state";
  import { goto } from "$app/navigation";
  import { enhanceForm, formatDateOnly } from "$lib/utils";
  import {
    completedStore,
    searchTerm,
    pageSize,
    currentPage,
    assigneeStore,
    sortBy,
    sortOrder,
  } from "$lib/stores";
  import { Bell, Eye, EyeClosed } from "@lucide/svelte";
  import { m } from "$lib/paraglide/messages";
  import message from "$lib/messages.js";

  const { data } = $props();
  const role = sveltePage.params.role;

  $effect(() => {
    const params = new URLSearchParams();

    if ($searchTerm?.trim()) params.set("search", $searchTerm);
    if ($pageSize > 0) params.set("page_size", $pageSize.toString());
    if ($currentPage > 1) params.set("page", $currentPage.toString());
    if ($assigneeStore?.trim()) params.set("assignee", $assigneeStore);
    if ($completedStore) params.set("completed", String($completedStore));
    if ($sortBy?.trim()) params.set("sort_by", $sortBy);
    if ($sortOrder?.trim()) params.set("sort_order", $sortOrder);
    const queryString = params.toString();
    const newUrl = queryString ? `?${queryString}` : window.location.pathname;

    goto(newUrl, {
      noScroll: true,
      keepFocus: true,
    });
  });

  function toggleCompletedTasks() {
    completedStore.set(!$completedStore);
  }
  const { page, totalPages, count, perPage } = data.tasksPaginated;
  const tasks = $derived(data.tasksPaginated.data);
  let selected: string[] = $state([]);
</script>

<svelte:head>
  <title>{m.tasks()}</title>
</svelte:head>

<Toolbar>
  <LargeTitle>{m.tasks()}</LargeTitle>
  <Divider />
  <VStack>
    <Merger>
      {#if role == "t"}
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
      {:else if role == "s"}
        <form
          action="?/requestHW"
          method="post"
          use:enhance={enhanceForm({
            messages: {
              success: message.tasks.teacherNotified,
            },
          })}
        >
          <UniButton
            content={m.tense_mealy_kitten_aid()}
            Icon={Bell}
            type="submit"
            variant="primary"
          ></UniButton>
        </form>
      {/if}
    </Merger>

    <Merger>
      <UniButton
        type="button"
        content={$completedStore === true
          ? m.steep_zany_tern_zip()
          : m.direct_slow_bobcat_shine()}
        onclick={toggleCompletedTasks}
        variant="primary"
        Icon={$completedStore === true ? EyeClosed : Eye}
      ></UniButton>
    </Merger>
    <SearchBar bind:q={$searchTerm} />
  </VStack>
</Toolbar>

{#if data.tasksPaginated.data.length < 1}
  <EmptySpace>
    <Title1>{m.empty()}</Title1>
  </EmptySpace>
{/if}
{#if role === "s"}
  <div class="grid grid-cols-1 gap-4 md:grid-cols-3">
    {#each data.tasksPaginated.data as task (task.id)}
      <TaskCard {task} />
    {/each}
  </div>
{:else}
  <Table>
    <input type="hidden" bind:value={selected} name="toDelete" />
    <TableHead>
      <TickMorph
        noText={true}
        bind:group={selected}
        value={tasks.map((task) => task.id)}
      />
      {#if selected.length >= 1}
        <Subheadline>
          Выбрано {selected.length} из {tasks.length}
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
      {#each tasks as task (task.id)}
        <div class="bg-clickable flex items-center px-2">
          <TickMorph noText={true} bind:group={selected} value={task.id} />
          <TableRow href={`/${sveltePage.params.role}/tasks/${task.id}`}>
            <HStack override="gap-1 items-start">
              <Headline>
                {task.title}
              </Headline>
              <Caption1>
                {task.assigneeName}
              </Caption1>
            </HStack>
            <Divider />
            <Caption1>
              {formatDateOnly(task.dueDate)}
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
