<script lang="ts">
  import { enhance } from "$app/forms";

  import { Ban, Trash2, Check } from "lucide-svelte";
  import { enhanceForm } from "$lib/utils";
  import {
    Editor,
    H1,
    UniButton,
    AssigneeSelector,
    Toggler,
    HeaderEmbellish,
    GreySpan,
    Multipart,
    Input,
  } from "$lib/components";
  import type { PageData } from "./$types";

  let { data }: { data: PageData } = $props();
  let { task, files } = data;

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
      redirect: "Changes Saved",
      error: "Error",
      failure: "Something's off",
    },
  })}
>
  <HeaderEmbellish>
    <div class="flex items-end space-x-4">
      <H1>Edit Task</H1>
      <GreySpan>
        {files.length}
        {files.length === 1 ? "file" : "files"}
      </GreySpan>
    </div>
    <div class="flex items-center space-x-3">
      <UniButton variant="secondary" Icon={Ban} href=".">Cancel</UniButton>
      <UniButton variant="primary" type="submit" Icon={Check}>Save</UniButton>
      <UniButton
        variant="danger"
        Icon={Trash2}
        formaction="?/delete"
        confirmText={task.title}
        confirmTitle="Delete Task">Delete</UniButton
      >
    </div>
  </HeaderEmbellish>

  <input type="hidden" name="initialAssignee" value={task.assignee} />
  <input type="hidden" name="markdown" value={markdown} />

  <div class="grid grid-cols-1 gap-5 md:grid-cols-4">
    <Input name="title" value={task.title} placeholder="Title"></Input>
    <AssigneeSelector item={task} />

    <Input
      bind:value={dueDate}
      type="date"
      name="dueDate"
      labelName="Due Date"
      placeholder="Due Date"
    ></Input>
    <div class="mt-2 self-end">
      <Toggler value={task.completed} />
    </div>
  </div>
</form>

<div class="grid grid-cols-3 gap-4 space-x-4">
  <Editor bind:markdownContent={markdown} />
  <Multipart taskId={task.id} />
</div>
