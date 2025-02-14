<script lang="ts">
  import { enhance } from "$app/forms";
  import { Editor, ButtonDelete, ButtonSubmit, H1 } from "$lib/components";
  import type { PageData } from "./$types";
  import { notification } from "$lib/stores";
  import { goto } from "$app/navigation";
  let { data }: { data: PageData } = $props();
  let { lesson, students } = data;
  let isSubmitting = $state(false);
  let markdown = $state(lesson.markdown);
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
    <H1>Edit Lesson</H1>
    <a
      href="."
      class="text-milk-700 bg-milk-100 hover:bg-milk-200 rounded-lg px-4 py-2 transition-colors"
    >
      Cancel
    </a>
    <ButtonSubmit />
    <ButtonDelete />
  </div>

  <input type="hidden" name="id" value={lesson.id} />
  <input type="hidden" name="markdown" value={markdown} />
  <div class="flex space-x-4">
    <div class="space-y-2">
      <label for="title" class="text-milk-700 block font-medium">Title</label>
      <input
        id="title"
        type="text"
        name="title"
        class="dark:focus:ring-milk-700 dark:focus:border-milk-800 dark:border-milk-800 disabled:text-milk-500 border-milk-200 dark:bg-milk-950 focus:ring-cacao-500 w-full rounded-lg border px-4 py-2
            transition duration-200 focus:ring-2 focus:outline-none
                   dark:focus:ring-2 dark:focus:outline-none"
        value={lesson.title}
      />
    </div>

    <div class="space-y-2">
      <label for="topic" class="text-milk-700 block font-medium">Topic</label>
      <input
        id="topic"
        type="text"
        name="topic"
        value={lesson.topic}
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
        name="assignee"
        value={lesson.assignee}
        class="border-milk-200 focus:border-cacao-500 focus:ring-cacao-500 dark:bg-milk-950 w-full rounded-lg
				shadow-sm
				"
      >
        <option value="">Select an assignee</option>
        {#each students as student}
          <option value={student.id} selected={student.id === lesson.assignee}>
            {student.name}
          </option>
        {/each}
      </select>
    </div>
  </div>
</form>
<Editor bind:markdownContent={markdown} />
