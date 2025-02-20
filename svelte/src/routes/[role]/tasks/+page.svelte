<script lang="ts">
  import { user } from "$lib/stores";
  import type { Task, TableConfig } from "$lib/types";
  import { H1, Table, ButtonSubmit, TaskCard, H2 } from "$lib/components";
  import { formatDateTime } from "$lib/utils";
  import { enhance } from "$app/forms";
  import { page } from "$app/state";
  import { slide } from "svelte/transition";
  import { notification } from "$lib/stores";

  let { data } = $props();

  let role = $derived(page.params.role);
  let isSubmitting = $state(false);
  const { tasks, students } = data;

  const taskConfig: TableConfig<Task> = {
    columns: [
      { key: "title", label: "Title" },
      { key: "markdown", label: "Markdown" },
      {
        key: "completed",
        label: "Done",
        formatter: (value: boolean) => (value ? "✅" : "⌛"),
      },
      {
        key: "dueDate",
        label: "Due",
        formatter: (value: string) =>
          value ? formatDateTime(value) : "No Due Date",
      },
      {
        key: "assigneeName",
        label: "Assignee",
        formatter: (value: string) =>
          value === $user.name ? "Not Assigned" : value,
      },
    ],
  };

  let href = `/${role}/tasks/t`;
  let pending = tasks.filter((task) => !task.completed);
  let completed = tasks.filter((task) => task.completed);
  let completedVisible = $state(false);

  $effect(() => {
    pending.sort(
      (a, b) =>
        new Date(b.updatedAt).getTime() - new Date(a.updatedAt).getTime(),
    );
    completed.sort(
      (a, b) =>
        new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime(),
    );
  });
</script>

<H1>Tasks</H1>

{#if role === "t"}
  <Table items={tasks} config={taskConfig} {href} {students} />

  <form action="?/new" method="post" use:enhance>
    {#if tasks.length === 0}
      <ButtonSubmit buttonName="Add your first one!" />
    {/if}
  </form>
{:else}
  <section class="space-y-4">
    <H2>Active Tasks ({pending.length})</H2>
    {#if pending.length > 0}
      <div class="grid grid-cols-1 gap-4 md:grid-cols-2">
        {#each pending as task (task.id)}
          <TaskCard {task} interactive={true} />
        {/each}
      </div>
    {:else}
      <form
        method="POST"
        action="?/requestHW"
        use:enhance={() => {
          isSubmitting = true;

          return async ({ result }) => {
            if (result.type === "success") {
              notification.set({ message: "Notified", type: "success" });
            } else {
              notification.set({
                message: "Failed to Notify",
                type: "error",
              });
            }
            isSubmitting = false;
          };
        }}
      >
        <input type="hidden" value={$user.username} name="username" />
        <ButtonSubmit buttonName="I Want More" bind:isSubmitting />
      </form>
    {/if}
  </section>

  <!-- Completed Tasks Toggle & Section -->
  <section class="space-y-4">
    <button
      class="text-milk-700 hover:text-milk-900 inline-flex items-center space-x-2 transition-colors"
      onclick={() => (completedVisible = !completedVisible)}
    >
      <span class="text-lg font-medium"
        >Completed Tasks ({completed.length})</span
      >
      <svg
        class="h-5 w-5 transform transition-transform duration-200"
        class:rotate-180={completedVisible}
        viewBox="0 0 20 20"
        fill="currentColor"
      >
        <path
          d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z"
        />
      </svg>
    </button>

    {#if completedVisible}
      <div
        transition:slide={{ duration: 300 }}
        class="grid grid-cols-1 gap-6 md:grid-cols-2"
      >
        {#each completed as task (task.id)}
          <TaskCard {task} interactive />
        {/each}
      </div>
    {/if}
  </section>
{/if}

<svelte:head>
  <title>Tasks</title>
</svelte:head>
