<script lang="ts">
  import { enhance } from "$app/forms";

  import { enhanceForm } from "$lib/utils";
  import {
    Editor,
    LargeTitle,
    Toolbar,
    Multipart,
    Input,
    Merger,
    Divider,
    CancelButton,
    DeleteButton,
    SaveButton,
  } from "$lib/components";
  import type { PageData } from "./$types";
  import { m } from "$lib/paraglide/messages";
  import VStack from "$lib/components/UI/layout/VStack.svelte";

  let { data, form } = $props();
  let { task } = data;

  let markdown = $state(task.markdown);

  let dueDate = $state(
    task.dueDate ? new Date(task.dueDate).toISOString().split("T")[0] : "",
  );

  $effect(() => {
    if (task.dueDate) {
      dueDate = new Date(task.dueDate).toISOString().split("T")[0];
    }
  });
</script>

<form
  method="POST"
  action="?/update"
  class="mb-4 space-y-6"
  use:enhance={enhanceForm({
    messages: {
      redirect: m.changesSaved(),
    },
  })}
>
  <Toolbar>
    <LargeTitle>{m.editing()}</LargeTitle>
    <Divider />
    <VStack>
      <Merger>
        <CancelButton />
        <DeleteButton />
      </Merger>
      <Merger>
        <SaveButton />
      </Merger>
    </VStack>
  </Toolbar>

  <input type="hidden" name="markdown" value={markdown} />

  <div class="grid grid-cols-1 gap-5 md:grid-cols-3">
    <Input
      name="title"
      labelName="Название"
      value={task.title}
      placeholder="Title"
    ></Input>
    <Input
      name="assignee"
      labelName="Назначено"
      type="assignee"
      item={task}
      placeholder="Для кого задание"
    />

    <Input
      bind:value={dueDate}
      type="date"
      name="dueDate"
      invalidDescription="Что-то не так со временем"
      labelName="Срок выполнения"
      placeholder="Due Date"
    ></Input>
  </div>
</form>

<div class="grid gap-4 md:grid-cols-3">
  <Editor bind:markdownContent={markdown} />
  <Multipart taskId={task.id} />
</div>
