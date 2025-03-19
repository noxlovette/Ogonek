<script lang="ts">
  import { user } from "$lib/stores";
  import type { Task, TableConfig } from "$lib/types/index.js";
  import { H1, Table, TaskCard, H2, UniButton } from "$lib/components";

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
  import { Eye, EyeClosed, PlusCircle, Search } from "lucide-svelte";
  import { formatDate } from "@noxlovette/svarog";

  let { data } = $props();
  const { students } = data;
  let { data: tasks, total } = $derived(data.tasksPaginated);
  let role = page.params.role;
  let isSubmitting = $state(false);

  const taskConfig: TableConfig<Task> = {
    columns: [
      { key: "title", label: "Title" },
      { key: "markdown", label: "Markdown" },
      {
        key: "completed",
        label: "Done",
        formatter: (value: unknown): string => (value ? "✅" : "⌛"), // Explicitly return string
      },
      {
        key: "dueDate",
        label: "Due",
        formatter: (value: unknown): string =>
          value ? formatDate(value as string) : "No Due Date", // Explicitly return string
      },
      {
        key: "assigneeName",
        label: "Assignee",
        formatter: (value: unknown): string =>
          (value as string) === $user.name
            ? "You"
            : (value as string) || "Not Assigned", // Handle null/undefined
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

  // Fun phrases for the button
  const buttonPhrases = [
    "Feed Me Tasks! 🍔",
    "More Work Please! 🧠",
    "Task Drought... Help! 🏜️",
    "Bored Student Here! 📚",
    "Challenge Me! 💪",
    "My Brain Needs Exercise! 🏋️",
    "Send Homework My Way! 📝",
  ];

  // Pick a random phrase
  const randomPhrase =
    buttonPhrases[Math.floor(Math.random() * buttonPhrases.length)];
</script>

<H1>Tasks</H1>
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
    {#if tasks.length === 0}
      <UniButton type="submit" variant="ghost" Icon={PlusCircle}
        >Add your first one</UniButton
      >
      >
    {/if}
  </form>
{:else}
  <section class="space-y-4">
    <div class="flex items-center justify-between">
      <H2>Active Tasks ({tasks.length})</H2>

      <UniButton
        type="button"
        onclick={toggleCompletedTasks}
        variant="primary"
        Icon={$completedStore === true ? EyeClosed : Eye}
      >
        {$completedStore === true ? "Hide Completed" : "Show Completed"}
      </UniButton>
    </div>

    {#if tasks.length > 0}
      <div class="grid grid-cols-1 gap-4 md:grid-cols-2">
        {#each tasks as task (task.id)}
          <TaskCard {task} interactive={true} />
        {/each}
      </div>
    {:else}
      <div class="flex flex-col items-center justify-center py-12 text-center">
        <div class="relative">
          <div class="mb-6 rounded-full bg-stone-100 p-8 dark:bg-stone-800">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              class="text-cacao-400 dark:text-cacao-300 h-16 w-16"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M9.663 17h4.673M12 3v1m6.364 1.636l-.707.707M21 12h-1M4 12H3m3.343-5.657l-.707-.707m2.828 9.9a5 5 0 117.072 0l-.548.547A3.374 3.374 0 0014 18.469V19a2 2 0 11-4 0v-.531c0-.895-.356-1.754-.988-2.386l-.548-.547z"
              />
            </svg>
          </div>
        </div>

        <h3 class="mb-2 text-2xl font-bold text-stone-800 dark:text-stone-200">
          Task Inbox Zero!
        </h3>

        <p class="mb-6 max-w-md text-stone-600 dark:text-stone-400">
          Wow, you've completed all your tasks! Time to either celebrate or ask
          for more challenges.
        </p>

        <form
          method="POST"
          action="?/requestHW"
          class="w-full max-w-xs"
          use:enhance={enhanceForm({
            messages: {
              success: "Teacher Notified",
              error: "Error",
              failure: "Something's off",
            },
          })}
        >
          <input type="hidden" value={$user.username} name="username" />
          <button
            type="submit"
            disabled={isSubmitting}
            class="group from-cacao-500 to-cacao-600 hover:from-cacao-600 hover:to-cacao-700 focus:ring-cacao-500 relative w-full overflow-hidden rounded-lg bg-gradient-to-r px-6
             py-3 text-white shadow-sm ring ring-stone-200 transition-all focus:ring-2 focus:ring-offset-2 focus:outline-none
             disabled:opacity-70 dark:ring-stone-900"
          >
            <span class="relative z-10 flex items-center justify-center gap-2">
              {#if isSubmitting}
                <svg
                  class="h-5 w-5 animate-spin"
                  xmlns="http://www.w3.org/2000/svg"
                  fill="none"
                  viewBox="0 0 24 24"
                >
                  <circle
                    class="opacity-25"
                    cx="12"
                    cy="12"
                    r="10"
                    stroke="currentColor"
                    stroke-width="4"
                  ></circle>
                  <path
                    class="opacity-75"
                    fill="currentColor"
                    d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                  ></path>
                </svg>
                <span>Requesting...</span>
              {:else}
                <span>{randomPhrase}</span>
              {/if}
            </span>
          </button>
        </form>
      </div>
    {/if}
  </section>
{/if}

<svelte:head>
  <title>Tasks</title>
</svelte:head>
