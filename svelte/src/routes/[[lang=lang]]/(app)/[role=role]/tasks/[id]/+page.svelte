<script lang="ts">
  import {
    EmptySpace,
    H1,
    H3,
    Label,
    Toolbar,
    UniButton,
    FileTaskCard,
  } from "$lib/components";
  import { page } from "$app/state";
  import { enhance } from "$app/forms";
  import { CheckSquare, Square, Pencil } from "lucide-svelte";
  import { enhanceForm } from "$lib/utils";
  import { formatDate } from "@noxlovette/svarog";
  import Multipart from "$lib/components/UI/interactive/Multipart.svelte";
  import { user } from "$lib/stores/user";
  import Badge from "$lib/components/cards/Badge.svelte";
  import { getUrgency } from "$lib/utils";
  import Priority from "$lib/components/cards/Priority.svelte";
  import { m } from "$lib/paraglide/messages.js";

  let { data } = $props();
  const { files, rendered } = $derived(data);

  let role = $derived(page.params.role);
  let completed = $state(data.task.completed);

  let formattedDate = formatDate(data.task.createdAt);

  const urgency = getUrgency(data.task);
</script>

<svelte:head>
  <title>Task From {formattedDate}</title>
</svelte:head>

<Toolbar>
  <div class="flex flex-col gap-3 md:flex-row md:items-baseline md:gap-4">
    <H1>{data.task.title}</H1>
    {#if role === "t"}
      <H3>
        {data.task.assigneeName}
      </H3>
    {/if}
    <div class="flex items-center justify-center gap-4">
      <Priority priority={data.task.priority}></Priority>
      <Badge badgeText={formattedDate} {urgency}></Badge>
    </div>
  </div>
  <div class="flex items-center gap-3 md:gap-4">
    <form
      class="flex"
      method="post"
      action="?/complete"
      use:enhance={enhanceForm({
        messages: {
          success: completed ? m.notCompleted() : m.markedAsCompleted(),
          defaultError: m.failedToSaveChanges(),
        },
        handlers: {
          success: async () => {
            completed = !completed;
          },
        },
      })}
    >
      <UniButton
        variant="primary"
        type="submit"
        Icon={completed ? CheckSquare : Square}
      >
        {#if completed}
          <p class="">{m.completed()}</p>
        {:else}
          <p class="">{m.notCompleted()}</p>
        {/if}
      </UniButton>

      <input type="hidden" name="completed" value={!completed} />
      <input type="hidden" name="id" value={data.task.id} />
      <input type="hidden" name="task" value={data.task.title} />
      <input type="hidden" name="username" value={$user.username} />
    </form>
    {#if role === "t"}
      <UniButton
        Icon={Pencil}
        href="/t/tasks/{data.task.id}/edit"
        variant="secondary">{m.edit()}</UniButton
      >
    {/if}
  </div>
</Toolbar>

<grid class="grid gap-4 md:grid-cols-4">
  <div class="flex gap-4 md:flex-col">
    {#if files.length > 0}
      <div class="flex w-full flex-col gap-4">
        <Label>{m.stock_wise_cowfish_roam()}</Label>
        {#each files as file (file.id)}
          <FileTaskCard {file} />
        {/each}
      </div>
    {:else}
      <EmptySpace>{m.sleek_empty_zebra_harbor()}</EmptySpace>
    {/if}
    {#if page.params.role === "s"}
      <div class="flex w-full flex-col space-y-2">
        <Label>{m.bright_helpful_firefox_stir()}</Label>
        <Multipart taskId={data.task.id} />
      </div>
    {/if}
  </div>
  <div class="markdown md:col-span-3">
    <!-- Input is sanitized with rehype -->
    {@html rendered}
  </div>
</grid>
