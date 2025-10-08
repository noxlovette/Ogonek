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
  } from "$lib/components";
  import { enhance } from "$app/forms";
  import { Check, Circle } from "@lucide/svelte";
  import { enhanceForm, formatDateOnly } from "$lib/utils";
  import Badge from "$lib/components/cards/Badge.svelte";
  import { getUrgency } from "$lib/utils";
  import VStack from "$lib/components/UI/layout/VStack.svelte";
  import DownloadButton from "$lib/components/UI/forms/buttons/DownloadButton.svelte";
  import texts from "$lib/texts.js";

  let { data, form } = $props();
  const { files, rendered, task } = $derived(data);

  let completed = $state(task.completed);

  let formattedDate = $state();

  if (task.dueDate) {
    formattedDate = formatDateOnly(task.dueDate);
  }

  const urgency = getUrgency(task.dueDate);
</script>

<svelte:head>
  <title>Task • {task.title}</title>
</svelte:head>

<Toolbar override={true}>
  <HStack>
    <VStack>
      <LargeTitle>{task.title}</LargeTitle>
      <Divider />
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
              success: completed
                ? texts.tasks.notCompleted
                : texts.tasks.completed,
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
            content={completed ? texts.crud.uncomplete : texts.crud.complete}
            Icon={completed ? Check : Circle}
          ></UniButton>
        </form>
      </Merger>
    </VStack>
    <VStack override="gap-2">
      <VStack>
        {#if !task?.completed}
          <Badge {urgency}>{formattedDate}</Badge>
        {:else}
          <Badge urgency="green">Выполнено</Badge>
        {/if}
      </VStack>
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
        <Caption1>Прикрепленные файлы</Caption1>
        {#each files as file (file.id)}
          <FileTaskCard {file} />
        {/each}
      </div>
    {:else}
      <EmptySpace>{texts.table.empty}</EmptySpace>
    {/if}
  </div>
</grid>
