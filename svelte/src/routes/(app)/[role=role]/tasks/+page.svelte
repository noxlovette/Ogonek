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
    Badge,
  } from "$lib/components";

  import { enhance } from "$app/forms";
  import { page as sveltePage } from "$app/state";
  import { goto } from "$app/navigation";
  import { enhanceForm, formatDateOnly, getUrgency } from "$lib/utils";
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
  import message, { texts } from "$lib/texts.js";

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
    currentPage.set(1);
  }
  const { page, totalPages, count, perPage } = $derived(data.tasksPaginated);
  const tasks = $derived(data.tasksPaginated.data);
  let selected: string[] = $state([]);
</script>

<Toolbar>
  <LargeTitle>{texts.tasks.title}</LargeTitle>
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
              success: texts.notifications.homeworkRequested,
            },
          })}
        >
          <UniButton
            content={texts.crud.save}
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
          ? texts.crud.complete
          : texts.crud.uncomplete}
        onclick={toggleCompletedTasks}
        variant="primary"
        Icon={$completedStore === true ? EyeClosed : Eye}
      ></UniButton>
    </Merger>
    <SearchBar bind:q={$searchTerm} />
  </VStack>
</Toolbar>

{#if tasks.length}
  {#if role === "s"}
    <div class="grid grid-cols-1 gap-4 md:grid-cols-3">
      {#each data.tasksPaginated.data as task (task.id)}
        <TaskCard {task} />
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
          value={tasks.map((task) => task.id)}
        />
        {#if selected.length >= 1}
          <Subheadline>
            Выбрано {selected.length} из {tasks.length}
          </Subheadline>
        {:else}
          <Subheadline>{texts.table.selectAll}</Subheadline>
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
                  {task.assigneeName ? task.assigneeName : task.visibility}
                </Caption1>
              </HStack>
              <Divider />

              {#if !task?.completed}
                <Badge urgency={getUrgency(task.dueDate)}>
                  {formatDateOnly(task.dueDate)}
                </Badge>
              {:else}
                <Badge urgency="green">Выполнено</Badge>
              {/if}
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
    {texts.table.empty}
  </EmptySpace>
{/if}

<svelte:head>
  <title>{texts.tasks.title}</title>
</svelte:head>
