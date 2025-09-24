<script lang="ts">
  import type { TaskFull, TableConfig } from "$lib/types/index.js";
  import {
    LargeTitle,
    Divider,
    Merger,
    Table,
    UniButton,
    Toolbar,
    SearchBar,
    TableSkeleton,
    LoadingCard,
    TaskCard,
    EmptySpace,
    VStack,
    Title1,
  } from "$lib/components";

  import { enhance } from "$app/forms";
  import { page } from "$app/state";
  import { goto } from "$app/navigation";
  import { enhanceForm } from "$lib/utils";
  import {
    completedStore,
    searchTerm,
    pageSize,
    currentPage,
    assigneeStore,
  } from "$lib/stores";
  import { Bell, Eye, EyeClosed, Plus, PlusCircle } from "lucide-svelte";
  import { formatDate } from "$lib/utils";
  import { m } from "$lib/paraglide/messages";
  import message from "$lib/messages.js";
  import NewButton from "$lib/components/UI/forms/buttons/NewButton.svelte";

  const { data } = $props();
  const role = page.params.role;

  const taskConfig: TableConfig<TaskFull> = {
    columns: [
      { key: "title", label: m.title() },
      {
        key: "assigneeName",
        label: m.assignee(),
        formatter: (value: unknown): string =>
          (value as string) || m.notAssigned(),
      },
      {
        key: "dueDate",
        label: m.less_arable_starfish_belong(),
        formatter: (value: unknown): string =>
          value ? formatDate(value as string) : m.arable_flat_emu_strive(),
      },
    ],
  };

  let href = `/${role}/tasks`;
  $effect(() => {
    const params = new URLSearchParams();

    if ($searchTerm?.trim()) params.set("search", $searchTerm);
    if ($pageSize > 0) params.set("page_size", $pageSize.toString());
    if ($currentPage > 1) params.set("page", $currentPage.toString());
    if ($assigneeStore?.trim()) params.set("assignee", $assigneeStore);
    if ($completedStore) params.set("completed", String($completedStore));

    const queryString = params.toString();
    const newUrl = queryString ? `?${queryString}` : window.location.pathname;

    goto(newUrl, {
      noScroll: true,
      keepFocus: true,
    });
  });

  function toggleCompletedTasks() {
    completedStore.toggle();
  }
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

{#await data.tasksPaginated}
  {#if role === "s"}
    <div class="grid grid-cols-1 gap-4 md:grid-cols-3">
      <LoadingCard />
      <LoadingCard />
      <LoadingCard />
    </div>
  {:else}
    <TableSkeleton />
  {/if}
{:then tasks}
  {#if tasks.data.length < 1}
    <EmptySpace>
      <Title1>{m.noTasks()}</Title1>
    </EmptySpace>
  {/if}
  {#if role === "s"}
    <div class="grid grid-cols-1 gap-4 md:grid-cols-3">
      {#each tasks.data as task (task.id)}
        <TaskCard {task} />
      {/each}
    </div>
  {:else}
    <Table items={tasks.data} {href} config={taskConfig} />
  {/if}
{:catch error: App.Error}
  <p>Error loading lessons: {error.errorID}</p>
{/await}
