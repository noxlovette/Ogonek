<script lang="ts">
  import { H1, UniButton } from "$lib/components";
  import { user } from "$lib/stores";
  import { formatDateTime } from "$lib/utils";
  import { goto } from "$app/navigation";
  import { page } from "$app/state";

  import { enhance } from "$app/forms";
  import {
    Download,
    Loader2,
    CheckSquare,
    Square,
    Pencil,
  } from "lucide-svelte";
  import { enhanceForm } from "$lib/utils";

  let { data } = $props();
  const { rendered } = data;

  let role = $derived(page.params.role);
  let overdue = $state(false);
  let completed = $state(data.task.completed);

  let formattedDate = formatDateTime(data.task.createdAt);
  let isPreloading = $state(false);
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
      <p class="text-milk-700 block font-medium">Student</p>
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

  {#if data.task.filePath}
    <div>
      <UniButton
        Icon={Pencil}
        onclick={() => goto(`/download/${data.task.filePath}`)}
        variant="outline">Download</UniButton
      >
    </div>
  {/if}
{:else}
  <div class="flex items-baseline justify-between">
    <H1>{data.task.title}</H1>
  </div>

  <div class="markdown ring-milk-200 dark:ring-milk-900 rounded-lg p-4 ring">
    {@html rendered}
  </div>
  <div class="flex space-x-3">
    {#if data.task.filePath}
      <a
        href="/download/{data.task.filePath}"
        onclick={() => (isPreloading = true)}
        class="dark:hover:bg-milk-900 ring-milk-200 dark:ring-milk-800 dark:hover:ring-milk-950 bg-cacao-600 text-cacao-50 dark:bg-milk-800 dark:text-cacao-100 hover:bg-cacao-700 focus:ring-cacao-500 flex items-center justify-center rounded-lg px-3 py-2 text-sm ring transition-all focus:ring focus:ring-offset-2 focus:outline-none disabled:cursor-not-allowed disabled:opacity-50 md:space-x-2 md:px-4 md:text-base"
      >
        {#if !isPreloading}
          <Download class="size-6" />
          <p class="hidden md:block">Download</p>
        {:else}
          <Loader2 class="animate-spin" />
          <p class="hidden md:block">On it...</p>
        {/if}
      </a>
    {/if}
    <form
      class="flex"
      method="post"
      use:enhance={enhanceForm({
        messages: {
          success: completed ? "Marked As Completed" : "Not Completed",
          defaultError: "Failed to mark as completed",
        },
      })}
    >
      <button
        onclick={() => (completed = !completed)}
        class="dark:hover:bg-milk-900 ring-milk-200 dark:ring-milk-800 dark:hover:ring-milk-950 bg-cacao-600 text-cacao-50 dark:bg-milk-800 dark:text-cacao-100 hover:bg-cacao-700 focus:ring-cacao-500 flex items-center justify-center rounded-lg px-3 py-2 text-sm ring transition-all focus:ring focus:ring-offset-2 focus:outline-none disabled:cursor-not-allowed disabled:opacity-50 md:space-x-2 md:px-4 md:text-base"
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
