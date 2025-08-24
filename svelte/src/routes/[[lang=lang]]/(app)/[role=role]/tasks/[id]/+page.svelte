<script lang="ts">
  import {
    EmptySpace,
    H1,
    H3,
    Label,
    Toolbar,
    UniButton,
    FileTaskCard,
    HStack,
    Caption,
    Merger,
    Divider,
  } from "$lib/components";
  import { page } from "$app/state";
  import { enhance } from "$app/forms";
  import {
    CheckSquare,
    Square,
    Pencil,
    Check,
    Circle,
    Divide,
  } from "lucide-svelte";
  import { enhanceForm } from "$lib/utils";
  import { formatDate } from "@noxlovette/svarog";
  import Multipart from "$lib/components/UI/interactive/Multipart.svelte";
  import { user } from "$lib/stores/user";
  import Badge from "$lib/components/cards/Badge.svelte";
  import { getUrgency } from "$lib/utils";
  import Priority from "$lib/components/cards/Priority.svelte";
  import { m } from "$lib/paraglide/messages.js";
  import VStack from "$lib/components/UI/toolbar/VStack.svelte";

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
  <HStack>
    <VStack>
      <H1>{data.task.title}</H1>
      <Divider />
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
            variant="primary"
            type="submit"
            Icon={completed ? Check : Circle}
          >
            {#if completed}
              <p class="">{m.completed()}</p>
            {:else}
              <p class="">{m.notCompleted()}</p>
            {/if}
          </UniButton>
        </form>
        {#if role === "t"}
          <UniButton Icon={Pencil} href="/t/tasks/{data.task.id}/edit"
            >{m.edit()}</UniButton
          >
        {/if}
      </Merger>
    </VStack>
    <VStack>
      <VStack>
        <Badge badgeText={formattedDate} {urgency}></Badge>
        <Priority priority={data.task.priority}></Priority>
      </VStack>
      {#if role === "t"}
        <Caption>
          {data.task.assigneeName}
        </Caption>
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
</grid>
