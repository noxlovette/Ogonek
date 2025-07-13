<script lang="ts">
  import type { Task, TableConfig } from "$lib/types/index.js";
  import { H1, Table, UniButton, HeaderEmbellish } from "$lib/components";

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
  import RequestHw from "$lib/components/UI/interactive/RequestHW.svelte";

  let { data } = $props();
  const { students } = data;
  let { data: tasks, total } = $derived(data.tasksPaginated);
  let role = page.params.role;

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
{#if role === "t"}
  <Table
    items={tasks}
    config={taskConfig}
    {href}
    {students}
    {total}
    showComplete={true}
  />
{:else}
  <RequestHw tasks={data.tasksPaginated.data} />
{/if}

<svelte:head>
  <title>{m.tasks()}</title>
</svelte:head>
