<script lang="ts">
  import { enhance } from "$app/forms";
  import { notification } from "$lib/stores";
  import {
    Editor,
    ButtonCancel,
    H1,
    ButtonDelete,
    ButtonSubmit,
    Uploader,
  } from "$lib/components";
  import type { PageData } from "./$types";

  let { data }: { data: PageData } = $props();
  let { task, students } = data;
  let isSubmitting = $state(false);
  let markdown = $state(task.markdown);
  let filePath = $state(task.filePath);
  let fileName = $state("");

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
  class="mb-4 space-y-4"
  use:enhance={() => {
    isSubmitting = true;
    return async ({ result, update }) => {
      isSubmitting = false;
      if (result.type === "redirect") {
        notification.set({ message: "Changes saved", type: "success" });
        update();
      } else {
        notification.set({
          message: "Failed to save changes",
          type: "error",
        });
      }
    };
  }}
>
  <div class="flex items-baseline space-x-4">
    <H1>Edit Task</H1>
    <ButtonCancel />
    <ButtonSubmit bind:isSubmitting />
    <ButtonDelete bind:isSubmitting />
  </div>

  <input type="hidden" name="id" value={task.id} />
  <input type="hidden" name="markdown" value={markdown} />
  <input type="hidden" name="filePath" value={filePath} />

  <div class="grid grid-cols-3 gap-4">
    <div class="space-y-2">
      <label for="title" class="text-milk-700 block font-medium">Title</label>
      <input
        id="title"
        type="text"
        name="title"
        value={task.title}
        class="dark:focus:ring-milk-700 dark:focus:border-milk-800 dark:border-milk-800 disabled:text-milk-500 border-milk-200 dark:bg-milk-950 focus:ring-cacao-500 w-full rounded-lg border px-4 py-2
            transition duration-200 focus:ring-2 focus:outline-none
                   dark:focus:ring-2 dark:focus:outline-none"
      />
    </div>
    <div class="space-y-2">
      <label for="assignee" class="text-milk-700 block font-medium"
        >Assignee</label
      >
      <select
        id="assignee"
        name="student"
        class="dark:focus:ring-milk-700 dark:focus:border-milk-800 dark:border-milk-800 disabled:text-milk-500 border-milk-200 dark:bg-milk-950 focus:ring-cacao-500 w-full rounded-lg border px-4 py-2
            transition duration-200 focus:ring-2 focus:outline-none
                   dark:focus:ring-2 dark:focus:outline-none"
      >
        <option value="">Select an assignee</option>
        {#each students as student}
          <option
            value={JSON.stringify({
              assignee: student.id,
              telegramId: student.telegramId,
            })}
            selected={student.id === task.assignee}
          >
            {student.name}
          </option>
        {/each}
      </select>
    </div>
    <div class="space-y-2">
      <label for="dueDate" class="text-milk-700 block font-medium"
        >Due Date</label
      >
      <input
        id="dueDate"
        type="date"
        name="dueDate"
        bind:value={dueDate}
        class="dark:focus:ring-milk-700 dark:focus:border-milk-800 dark:border-milk-800 disabled:text-milk-500 border-milk-200 dark:bg-milk-950 focus:ring-cacao-500 w-full rounded-lg border px-4 py-2
            transition duration-200 focus:ring-2 focus:outline-none
                   dark:focus:ring-2 dark:focus:outline-none"
      />
    </div>
    <div class="flex items-center space-y-2">
      <label class="relative inline-flex cursor-pointer items-center">
        <input
          type="checkbox"
          name="completed"
          checked={task.completed}
          class="peer sr-only"
        />
        <div
          class="bg-milk-200 peer-focus:ring-cacao-300 peer after:border-milk-300 peer-checked:bg-cacao-600
					   h-6 w-11 rounded-full
					   peer-focus:ring-4 peer-focus:outline-none
					   after:absolute after:top-[2px] after:left-[2px] after:h-5
					   after:w-5 after:rounded-full after:border after:bg-white
					   after:transition-all after:content-[''] peer-checked:after:translate-x-full peer-checked:after:border-white"
        ></div>
        <span class="text-milk-700 ml-3 text-sm font-medium">Completed</span>
      </label>
    </div>
  </div>
</form>

<div class="flex h-full w-full items-end space-x-4">
  <Editor bind:markdownContent={markdown} />
  <Uploader bind:filePath {fileName} />
</div>
