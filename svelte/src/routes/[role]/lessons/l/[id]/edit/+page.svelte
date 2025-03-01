<script lang="ts">
  import { enhance } from "$app/forms";
  import {
    Editor,
    H1,
    AssigneeSelector,
    UniButton,
    Label,
  } from "$lib/components";
  import type { PageData } from "./$types";
  import { notification } from "$lib/stores";
  import { enhanceForm } from "$lib/utils";
  import { Ban, Check, Trash2 } from "lucide-svelte";
  let { data }: { data: PageData } = $props();
  let { lesson } = data;

  let markdown = $state(lesson.markdown);
</script>

<form
  method="POST"
  action="?/update"
  class="mb-4 space-y-4"
  use:enhance={enhanceForm({
    messages: {
      redirect: "Changes Saved",
      defaultError: "Failed to save changes",
    },
    navigate: true,
  })}
>
  <div class="flex items-baseline space-x-4">
    <H1>Edit Lesson</H1>
    <UniButton variant="secondary" Icon={Ban} href=".">Cancel</UniButton>
    <UniButton variant="primary" type="submit" Icon={Check}>Save</UniButton>
    <UniButton
      variant="danger"
      formaction="?/delete"
      Icon={Trash2}
      confirmText={lesson.title}
      confirmTitle="Delete Lesson">Delete</UniButton
    >
  </div>

  <input type="hidden" name="id" value={lesson.id} />
  <input type="hidden" name="markdown" value={markdown} />
  <div class="flex space-x-4">
    <div class="space-y-2">
      <Label>Title</Label>
      <input
        id="title"
        type="text"
        name="title"
        class="dark:focus:ring-milk-700 dark:focus:border-milk-800 dark:border-milk-800 disabled:text-milk-500 border-milk-200 dark:bg-milk-950 focus:ring-cacao-500 w-full rounded-lg border px-4 py-2
            transition duration-200 focus:ring focus:outline-none
                   dark:focus:ring dark:focus:outline-none"
        value={lesson.title}
      />
    </div>

    <div class="space-y-2">
      <Label>Topic</Label>
      <input
        id="topic"
        type="text"
        name="topic"
        value={lesson.topic}
        class="dark:focus:ring-milk-700 dark:focus:border-milk-800 dark:border-milk-800 disabled:text-milk-500 border-milk-200 dark:bg-milk-950 focus:ring-cacao-500 w-full rounded-lg border px-4 py-2
            transition duration-200 focus:ring focus:outline-none
                   dark:focus:ring dark:focus:outline-none"
      />
    </div>
    <AssigneeSelector item={lesson} />
  </div>
</form>
<Editor bind:markdownContent={markdown} />
