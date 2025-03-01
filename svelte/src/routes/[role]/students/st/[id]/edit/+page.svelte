<script lang="ts">
  import { enhance } from "$app/forms";
  import { Editor, H1, UniButton } from "$lib/components";
  import type { Student } from "$lib/types";
  import { Ban, Check, Send, Trash2 } from "lucide-svelte";
  import type { PageData } from "./$types";
  import { enhanceForm } from "$lib/utils";
  let { data }: { data: PageData } = $props();
  let { student }: { student: Student } = data;
  let markdown = $state(student.markdown);
</script>

<form
  method="POST"
  action="?/update"
  class="mb-4 space-y-4"
  use:enhance={enhanceForm({
    messages: {
      redirect: "Changes Saved",
      error: "Error",
      failure: "Something's off",
    },
  })}
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

      <UniButton variant="secondary" Icon={Ban}>Cancel</UniButton>
      <UniButton variant="primary" Icon={Check}>Save</UniButton>
      <UniButton variant="danger" formaction="?/delete" Icon={Trash2}
        >Delete</UniButton
      >
    </div>

    <input type="hidden" name="id" value={student.id} />
    <input type="hidden" name="markdown" value={markdown} />
  </div>
</form>
<Editor bind:markdownContent={markdown} />
