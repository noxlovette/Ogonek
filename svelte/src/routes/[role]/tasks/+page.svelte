<script lang="ts">
  import { user } from "$lib/stores";
  import type { Task, TableConfig } from "$lib/types/index.js";
  import {
    H1,
    Table,
    TaskCard,
    H2,
    UniButton,
    HeaderEmbellish,
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
    teacherData,
  } from "$lib/stores";
  import { Eye, EyeClosed, Lightbulb, PlusCircle } from "lucide-svelte";
  import { formatDate } from "@noxlovette/svarog";

  $inspect($teacherData);

  let { data } = $props();
  const { students } = data;
  let { data: tasks, total } = $derived(data.tasksPaginated);
  let role = page.params.role;

  const taskConfig: TableConfig<Task> = {
    columns: [
      { key: "title", label: "Title" },
      {
        key: "assigneeName",
        label: "Assignee",
        formatter: (value: unknown): string =>
          (value as string) === $user.name
            ? "You"
            : (value as string) || "Not Assigned", // Handle null/undefined
      },
      {
        key: "dueDate",
        label: "Due",
        formatter: (value: unknown): string =>
          value ? formatDate(value as string) : "No Due Date", // Explicitly return string
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

  const buttonPhrases = [
    "Feed Me Tasks!",
    "More Work Please!",
    "Task Drought... Help!",
    "Bored Student Here!",
    "Challenge Me!",
    "My Brain Needs Exercise!",
    "Send Homework My Way!",
  ];

  const randomPhrase =
    buttonPhrases[Math.floor(Math.random() * buttonPhrases.length)];
</script>

<HeaderEmbellish>
  <H1>Tasks</H1>
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
  <form action="?/new" method="post" use:enhance>
    {#if tasks?.length === 0}
      <UniButton type="submit" variant="ghost" Icon={PlusCircle}
        >Add your first one</UniButton
      >
    {/if}
  </form>
{:else}
  <section class="space-y-4">
    <div class="flex items-center justify-between">
      <H2>Active Tasks ({tasks?.length})</H2>

      <UniButton
        type="button"
        onclick={toggleCompletedTasks}
        variant="primary"
        Icon={$completedStore === true ? EyeClosed : Eye}
      >
        {$completedStore === true ? "Hide Completed" : "Show Completed"}
      </UniButton>
    </div>

    {#if tasks?.length > 0}
      <div class="grid grid-cols-1 gap-4 md:grid-cols-2">
        {#each tasks as task (task.id)}
          <TaskCard {task} interactive={true} />
        {/each}
      </div>
    {:else}
      <div class="flex flex-col items-center justify-center py-12 text-center">
        <h3 class="mb-2 text-2xl font-bold text-stone-800 dark:text-stone-200">
          Task Inbox Zero
        </h3>

        <p class="mb-6 max-w-md text-stone-600 dark:text-stone-400">
          Wow, you've completed all your tasks! Time to either celebrate or ask
          for more challenges.
        </p>

        <form
          method="POST"
          action="?/requestHW"
          class=""
          use:enhance={enhanceForm({
            messages: {
              success: "Teacher Notified",
              error: "Error",
              failure: "Something's off",
            },
          })}
        >
          <input type="hidden" value={$user.username} name="username" />
          <input
            type="hidden"
            value={$teacherData.teacherTelegramId}
            name="teacherTelegramId"
          />
          <UniButton type="submit" variant="primary" Icon={Lightbulb}>
            {randomPhrase}
          </UniButton>
        </form>
      </div>
    {/if}
  </section>
{/if}

<svelte:head>
  <title>Tasks</title>
</svelte:head>
