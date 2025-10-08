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
  import { Check, Circle, Share } from "@lucide/svelte";
  import { enhanceForm, formatDateOnly } from "$lib/utils";
  import Multipart from "$lib/components/UI/interactive/Multipart.svelte";
  import Badge from "$lib/components/cards/Badge.svelte";
  import { getUrgency } from "$lib/utils";
  import VStack from "$lib/components/UI/layout/VStack.svelte";
  import DownloadButton from "$lib/components/UI/forms/buttons/DownloadButton.svelte";
  import texts from "$lib/texts.js";
  import { notification } from "$lib/stores/notification.js";

  let { data, form } = $props();
  const { files, rendered, task } = $derived(data);

  let role = $derived(page.params.role);
  let completed = $state(task.completed);

  let formattedDate = $state("Без даты");

  if (task.dueDate) {
    formattedDate = formatDateOnly(task.dueDate);
  }

  const urgency = getUrgency(task.dueDate);
</script>

<svelte:head>
  <title>Task • {task.title}</title>
</svelte:head>

<Toolbar>
  <HStack>
    <VStack>
      <LargeTitle>{task.title}</LargeTitle>
      <Divider />
      <Merger>
        {#if role === "t"}
          <EditButton href="/t/tasks/{task.id}/edit" />
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
        {#if role === "t"}
          <UniButton
            Icon={Share}
            onclick={async () => {
              await navigator.clipboard.writeText(
                `${window.location.origin}/share/task/${page.params.id}`,
              );

              notification.set({ message: "Есть ссылка", type: "success" });
            }}
            content="Поделиться занятием"
          />
        {/if}
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
            variant={role === "t" ? "primary" : "prominent"}
            type="submit"
            content={completed ? texts.crud.uncomplete : texts.crud.complete}
            Icon={completed ? Check : Circle}
          ></UniButton>
        </form>
      </Merger>
    </VStack>
    <VStack override="gap-2">
      <VStack>
        {#if !task.completed}
          <Badge {urgency}>{formattedDate}</Badge>
        {:else}
          <Badge urgency="green">Выполнено</Badge>
        {/if}
      </VStack>
      {#if role === "t"}
        <Badge>
          {task.assigneeName ? task.assigneeName : task.visibility}
        </Badge>
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
        <Caption1>Прикрепленные файлы</Caption1>
        {#each files as file (file.id)}
          <FileTaskCard {file} />
        {/each}
      </div>
    {:else}
      <EmptySpace>{texts.table.empty}</EmptySpace>
    {/if}
    {#if page.params.role === "s"}
      <div class="gap-default flex w-full flex-col">
        <Caption1>Вы можете загрузить здесь ДЗ</Caption1>
        <Multipart taskId={task.id} />
      </div>
    {/if}
  </div>
</grid>
