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
    Toggler,
  } from "$lib/components";
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

  let assigned = $state(task.assignee ? true : false);
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
      bind:value={dueDate}
      type="date"
      name="dueDate"
      invalid={form?.date}
      invalidDescription="Что-то не так со временем"
      labelName="Срок выполнения"
      placeholder="Due Date"
    ></Input>

    {#if assigned}
      <Input
        name="assignee"
        labelName="Назначено"
        type="assignee"
        item={task}
        invalid={form?.assignee}
        invalidDescription="Так для кого это?"
        placeholder="Для кого задание"
      />
    {/if}
  </div>
  <Toggler
    bind:value={assigned}
    name="assigned"
    title={assigned
      ? "Это задание будет привязано к ученику"
      : "Это задание ни к кому не будет привязано"}
  />
</form>

<div class="grid gap-4 md:grid-cols-3">
  <Editor bind:markdownContent={markdown} />
  <Multipart taskId={task.id} />
</div>
