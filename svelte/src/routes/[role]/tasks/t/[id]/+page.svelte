<script lang="ts">
  import {
    EmptySpace,
    H1,
    H3,
    HeaderEmbellish,
    UniButton,
  } from "$lib/components";
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
  let completed = $state(data.task.completed);

  let formattedDate = formatDate(data.task.createdAt);
</script>

<svelte:head>
  <title>Task From {formattedDate}</title>
</svelte:head>

<HeaderEmbellish>
  <div>
    <H1>{data.task.title}</H1>
    {#if role === "t"}
      <H3>
        {data.task.assigneeName}
      </H3>
    {/if}
  </div>
  <div class="flex items-center space-x-3">
    <form
      class="flex"
      method="post"
      action="?/complete"
      use:enhance={enhanceForm({
        messages: {
          success: completed ? "Not Completed" : "Marked As Completed",
          defaultError: "Failed to mark as completed",
        },
      })}
    >
      <UniButton
        variant="primary"
        type="submit"
        onclick={() => (completed = !completed)}
        Icon={completed ? CheckSquare : Square}
      >
        {#if completed}
          <p class="hidden md:block">Completed</p>
        {:else}
          <p class="hidden md:block">Mark as Completed</p>
        {/if}
      </UniButton>

      <input type="hidden" name="completed" value={completed} />
      <input type="hidden" name="id" value={data.task.id} />
    </form>
    {#if role === "t"}
      <UniButton
        Icon={Pencil}
        href="/t/tasks/t/{data.task.id}/edit"
        variant="outline">Edit</UniButton
      >
    {/if}
  </div>
</HeaderEmbellish>

<grid class="grid gap-4 md:grid-cols-4">
  <div class="markdown md:col-span-3">
    {@html rendered}
  </div>

  {#if files.length > 0}
    <div class="flex w-full flex-col items-center space-y-2">
      {#each files as file}
        <FileTaskCard {file} />
      {/each}
    </div>
  {:else}
    <EmptySpace>No files attached</EmptySpace>
  {/if}
</grid>
