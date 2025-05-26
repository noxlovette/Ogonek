<script lang="ts">
  import {
    EmptySpace,
    H1,
    H3,
    Label,
    HeaderEmbellish,
    UniButton,
    FileTaskCard,
  } from "$lib/components";
  import { page } from "$app/state";
  import { enhance } from "$app/forms";
  import { CheckSquare, Square, Pencil } from "lucide-svelte";
  import { enhanceForm } from "$lib/utils";
  import { formatDate } from "@noxlovette/svarog";
  import Multipart from "$lib/components/UI/interactive/Multipart.svelte";

  let { data } = $props();
  const { files, rendered } = $derived(data);

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
          <p class="">Completed</p>
        {:else}
          <p class="">Done</p>
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
    <!-- eslint-disable-next-line svelte/no-at-html -->
    <!-- Input is sanitized with rehype -->
    {@html rendered}
  </div>

  <div class="flex flex-col items-center space-y-2">
    {#if files.length > 0}
      <div class="flex w-full flex-col space-y-2">
        <Label>Attached Files</Label>
        {#each files as file}
          <FileTaskCard {file} />
        {/each}
      </div>
    {:else}
      <EmptySpace>No files attached</EmptySpace>
    {/if}
    {#if page.params.role === "s"}
      <div class="flex w-full flex-col space-y-2">
        <Label>Upload your HW here</Label>
        <Multipart taskId={data.task.id} notify={true} />
      </div>
    {/if}
  </div>
</grid>
