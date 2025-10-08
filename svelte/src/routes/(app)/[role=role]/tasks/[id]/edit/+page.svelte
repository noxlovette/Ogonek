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
    VStack,
    Popover,
  } from "$lib/components";
  import texts from "$lib/texts.js";

  let { data, form } = $props();
  let { task } = data;

  let markdown = $state(task.markdown);
  let visibility = $state(task.visibility);
  let dueDate = $state(
    task.dueDate ? new Date(task.dueDate).toISOString().split("T")[0] : "",
  );

  $effect(() => {
    if (task.dueDate) {
      dueDate = new Date(task.dueDate).toISOString().split("T")[0];
    }
  });

  let assigned = $derived(visibility === "shared");
</script>

<form
  method="POST"
  action="?/update"
  class="gap-default mb-4 flex flex-col"
  use:enhance={enhanceForm({
    messages: {
      redirect: texts.crud.updated,
    },
  })}
>
  <Toolbar>
    <LargeTitle>{texts.crud.editing}</LargeTitle>
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
  <input
    type="checkbox"
    name="assigned"
    checked={assigned}
    style="display: none;"
  />

  <VStack>
    <Input
      name="title"
      labelName="Название"
      value={task.title}
      invalid={form?.title}
      invalidDescription="Название?"
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

    <Divider />
    <Popover>
      <Input
        name="visibility"
        labelName="Кто видит"
        bind:value={visibility}
        type="visibility"
      />

      {#if visibility === "shared"}
        <Input
          name="assignee"
          placeholder="Для кого задание"
          labelName="Назначено"
          invalid={form?.assignee}
          invalidDescription="Для кого задание?"
          item={task}
          type="assignee"
        />
      {/if}
    </Popover>
  </VStack>
</form>

<div class="grid gap-4 md:grid-cols-3">
  <Editor bind:markdownContent={markdown} />
  <Multipart taskId={task.id} />
</div>
