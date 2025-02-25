<script lang="ts">
  import { enhance } from "$app/forms";
  import { ButtonCancel, ButtonSubmit, Editor, H1 } from "$lib/components";
  import type { Student } from "$lib/types";
  import { Send } from "lucide-svelte";
  import type { PageData } from "./$types";
  import { notification } from "$lib/stores";
  import ButtonDelete from "$lib/components/UI/buttons/ButtonDelete.svelte";
  let { data }: { data: PageData } = $props();
  let { student }: { student: Student } = data;
  let isSubmitting = $state(false);
  let markdown = $state(student.markdown);
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
  <div class="flex items-baseline justify-between">
    <H1>{student.name}</H1>
    <div class="flex items-center space-x-3">
      <div class="relative">
        <input
          type="text"
          name="telegramId"
          value={student.telegramId}
          placeholder="@username"
          class="border-milk-300 focus:ring-cacao-500 rounded-lg border py-2 pr-4 pl-10 focus:border-transparent focus:ring"
        />
        <span class="text-milk-400 absolute top-2.5 left-3">
          <Send></Send>
        </span>
      </div>

      <ButtonCancel />
      <ButtonSubmit bind:isSubmitting></ButtonSubmit>
      <ButtonDelete bind:isSubmitting></ButtonDelete>
    </div>

    <input type="hidden" name="id" value={student.id} />
    <input type="hidden" name="markdown" value={markdown} />
  </div>
</form>
<Editor bind:markdownContent={markdown} />
