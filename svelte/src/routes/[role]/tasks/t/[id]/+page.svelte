<script lang="ts">
  import { H1, UniButton } from "$lib/components";
  import { user } from "$lib/stores";
  import { page } from "$app/state";
  import { enhance } from "$app/forms";
  import { Download, CheckSquare, Square, Pencil } from "lucide-svelte";
  import { enhanceForm } from "$lib/utils";
  import { formatDate } from "@noxlovette/svarog";
  import FileTaskCard from "$lib/components/cards/FileTaskCard.svelte";

  let { data } = $props();
  const { task, files, rendered } = $derived(data);

  let role = $derived(page.params.role);
  let overdue = $state(false);
  let completed = $state(data.task.completed);

  let formattedDate = formatDate(data.task.createdAt);
</script>

<svelte:head>
  <title>Task From {formattedDate}</title>
</svelte:head>

{#if role === "t"}
  <div class="flex items-center justify-between">
    <div class="flex space-x-4">
      <H1>{data.task.title}</H1>

      <UniButton
        Icon={Pencil}
        href="/t/tasks/t/{data.task.id}/edit"
        variant="outline">Edit</UniButton
      >
    </div>
    <div class="text-right">
      <p class="block font-medium text-stone-700">Student</p>
      <h3 class="">
        {#if data.task.assigneeName === $user.username}
          Not Assigned
        {:else}
          {data.task.assigneeName}
        {/if}
      </h3>
    </div>
  </div>
  <div class="markdown">
    {@html rendered}
  </div>
{:else}
  <div class="flex items-baseline justify-between">
    <H1>{data.task.title}</H1>
  </div>

  <div class="markdown rounded-lg p-4 ring ring-stone-200 dark:ring-stone-900">
    {@html rendered}
  </div>
  <div class="flex space-x-3">
    <form
      class="flex"
      method="post"
      action="?/complete"
      use:enhance={enhanceForm({
        messages: {
          success: completed ? "Marked As Completed" : "Not Completed",
          defaultError: "Failed to mark as completed",
        },
      })}
    >
      <button
        onclick={() => (completed = !completed)}
        class="bg-cacao-600 text-cacao-50 dark:text-cacao-100 hover:bg-cacao-700 focus:ring-cacao-500 flex items-center justify-center rounded-lg px-3 py-2 text-sm ring ring-stone-200 transition-all focus:ring focus:ring-offset-2 focus:outline-none disabled:cursor-not-allowed disabled:opacity-50 md:space-x-2 md:px-4 md:text-base dark:bg-stone-800 dark:ring-stone-800 dark:hover:bg-stone-900 dark:hover:ring-stone-950"
        class:overdue
      >
        {#if completed}
          <CheckSquare class="h-6 w-6" />
          <p class="hidden md:block">Completed</p>
        {:else}
          <Square class="h-6 w-6" />
          <p class="hidden md:block">Mark as Completed</p>
        {/if}
      </button>
      <input type="hidden" name="completed" value={completed} />
      <input type="hidden" name="id" value={data.task.id} />
    </form>
  </div>
{/if}
<div class="flex space-x-4">
  {#each files as file}
    <FileTaskCard {file} />
  {/each}
</div>
