<script lang="ts">
  import { enhance } from "$app/forms";

  import { Ban, Trash2, Check } from "lucide-svelte";
  import { enhanceForm } from "$lib/utils";
  import {
    Editor,
    H1,
    UniButton,
    Uploader,
    AssigneeSelector,
    Label,
    Toggler,
    HeaderEmbellish,
    GreySpan,
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
    <div class="space-y-2">
      <Label>Title</Label>
      <input
        name="title"
        value={task.title}
        placeholder="Title"
        class="focus:ring-cacao-500 w-full rounded-lg border border-stone-200 px-4 py-2 transition duration-200 focus:ring focus:outline-none disabled:text-stone-500 dark:border-stone-800 dark:bg-stone-950 dark:focus:border-stone-800 dark:focus:ring dark:focus:ring-stone-700 dark:focus:outline-none"
      />
    </div>

    <div class="space-y-2">
      <AssigneeSelector item={task} />
    </div>

    <div class="space-y-2">
      <Label>Due Date</Label>
      <input
        id="dueDate"
        type="date"
        name="dueDate"
        bind:value={dueDate}
        class="focus:ring-cacao-500 w-full rounded-lg border border-stone-200 px-4 py-2 transition duration-200 focus:ring focus:outline-none disabled:text-stone-500 dark:border-stone-800 dark:bg-stone-950 dark:focus:border-stone-800 dark:focus:ring dark:focus:ring-stone-700 dark:focus:outline-none"
      />
    </div>
    <div class="mt-2 self-end">
      <Toggler bind:value={task.completed} />
    </div>
  </div>
</form>

<div class="flex space-x-4">
  <Editor bind:markdownContent={markdown} />
  <Uploader />
</div>
