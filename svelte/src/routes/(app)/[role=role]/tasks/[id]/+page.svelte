<script lang="ts">
  import {
    EmptySpace,
    LargeTitle,
    Toolbar,
    UniButton,
    FileTaskCard,
    HStack,
    Caption1,
    Merger,
    Divider,
    EditButton,
  } from "$lib/components";
  import { page } from "$app/state";
  import { enhance } from "$app/forms";
  import { Check, Circle } from "lucide-svelte";
  import { enhanceForm, formatDate } from "$lib/utils";
  import Multipart from "$lib/components/UI/interactive/Multipart.svelte";
  import Badge from "$lib/components/cards/Badge.svelte";
  import { getUrgency } from "$lib/utils";
  import { m } from "$lib/paraglide/messages.js";
  import VStack from "$lib/components/UI/layout/VStack.svelte";
  import DownloadButton from "$lib/components/UI/forms/buttons/DownloadButton.svelte";

  let { data, form } = $props();
  const { files, rendered } = $derived(data);

  let role = $derived(page.params.role);
  let completed = $state(data.task.completed);

  let formattedDate = $state(m.arable_flat_emu_strive());

  if (data.task.dueDate) {
    formattedDate = formatDate(data.task.dueDate);
  }

  const urgency = getUrgency(data.task);

  $inspect(form);
</script>

<svelte:head>
  <title>Task • {data.task.title}</title>
</svelte:head>

<Toolbar>
  <HStack>
    <VStack>
      <LargeTitle>{data.task.title}</LargeTitle>
      <Divider />
      <Merger>
        {#if role === "t"}
          <EditButton href="/t/tasks/{data.task.id}/edit" />
        {/if}
      </Merger>
      <Merger>
        <form
          class="flex"
          method="POST"
          action="?/downloadAll"
          use:enhance={enhanceForm({
            messages: {
              success: "Загрузка началась",
            },
            shouldUpdate: true,
          })}
        >
          <DownloadButton urls={form?.urls}></DownloadButton>
        </form>
      </Merger>
      <Merger>
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
            variant="prominent"
            type="submit"
            content={completed ? m.complete() : m.notCompleted()}
            Icon={completed ? Check : Circle}
          ></UniButton>
        </form>
      </Merger>
    </VStack>
    <VStack>
      <VStack>
        <Badge {urgency}>{formattedDate}</Badge>
      </VStack>
      {#if role === "t"}
        <Caption1>
          {data.task.assigneeName}
        </Caption1>
      {/if}
    </VStack>
  </HStack>
</Toolbar>

<grid class="grid gap-4 md:grid-cols-4">
  <div class="markdown md:col-span-3">
    {@html rendered}
  </div>
  <div class="flex gap-4 md:flex-col">
    {#if files.length > 0}
      <div class="gap-default flex w-full flex-col">
        <Caption1>{m.stock_wise_cowfish_roam()}</Caption1>
        {#each files as file (file.id)}
          <FileTaskCard {file} />
        {/each}
      </div>
    {:else}
      <EmptySpace>{m.sleek_empty_zebra_harbor()}</EmptySpace>
    {/if}
    {#if page.params.role === "s"}
      <div class="gap-default flex w-full flex-col">
        <Caption1>{m.bright_helpful_firefox_stir()}</Caption1>
        <Multipart taskId={data.task.id} />
      </div>
    {/if}
  </div>
</grid>
