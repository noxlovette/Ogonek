<script lang="ts">
  import type { Task, TableConfig } from "$lib/types/index.js";
  import {
    H1,
    Table,
    UniButton,
    HeaderEmbellish,
    SearchBar,
    TableSkeleton,
    LoadingCard,
    TaskCard,
    EmptySpace,
    H3,
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
  import { Eye, EyeClosed, PlusCircle } from "lucide-svelte";
  import { formatDate } from "@noxlovette/svarog";
  import { m } from "$lib/paraglide/messages";

  const { data } = $props();
  const { students } = data;
  const role = page.params.role;

  const taskConfig: TableConfig<Task> = {
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

  let href = `/${role}/tasks/t`;

  $effect(() => {
    goto(
      `?search=${$searchTerm}&page_size=${$pageSize}&page=${$currentPage}&assignee=${$assigneeStore}&completed=${$completedStore}`,
      {
        noScroll: true,
        keepFocus: true,
      },
    );
  });

  function toggleCompletedTasks() {
    completedStore.toggle();
  }
</script>

<HeaderEmbellish>
  <div
    class="flex flex-col items-center gap-3 md:flex-row md:items-center md:gap-4"
  >
    <H1>{m.tasks()}</H1>
    {#if role == "t"}
      <form
        action="?/new"
        method="post"
        use:enhance={enhanceForm({
          messages: {
            redirect: m.few_big_dachshund_scoop(),
          },
          navigate: true,
        })}
      >
        <UniButton Icon={PlusCircle} type="submit" variant="primary"
          >{m.new()}</UniButton
        >
      </form>
    {/if}

    <UniButton
      type="button"
      onclick={toggleCompletedTasks}
      variant="primary"
      Icon={$completedStore === true ? EyeClosed : Eye}
    >
      {$completedStore === true
        ? m.steep_zany_tern_zip()
        : m.direct_slow_bobcat_shine()}
    </UniButton>
  </div>
</HeaderEmbellish>

<SearchBar />
{#await data.tasksPaginated}
  {#if role === "s"}
    <div class="grid grid-cols-1 gap-4 md:grid-cols-3">
      {#each Array(6) as _, index (index)}
        <LoadingCard />
      {/each}
    </div>
  {:else}
    <TableSkeleton />
  {/if}
{:then tasks}
  {#if tasks.data.length < 1}
    <EmptySpace>
      <H3>{m.noTasks()}</H3>
    </EmptySpace>
  {/if}
  {#if role === "s"}
    <div class="grid grid-cols-1 gap-4 md:grid-cols-3">
      {#each tasks.data as task (task.id)}
        <TaskCard {task} />
      {/each}
    </div>
  {:else}
    <Table items={tasks.data} total={tasks.total} {href} config={taskConfig} />
  {/if}
{:catch error: App.Error}
  <p>Error loading lessons: {error.errorID}</p>
{/await}

<svelte:head>
  <title>{m.tasks()}</title>
</svelte:head>
