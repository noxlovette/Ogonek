<script lang="ts">
  import { enhance } from "$app/forms";
  import {
    Editor,
    H1,
    HeaderEmbellish,
    UniButton,
    Input,
  } from "$lib/components";
  import type { Student } from "$lib/types";
  import { Ban, Check, Trash2 } from "lucide-svelte";
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
  <HeaderEmbellish>
    <H1>{student.name}</H1>
    <div class="flex items-center space-x-3">
      <UniButton variant="secondary" href="." Icon={Ban}>Cancel</UniButton>
      <UniButton variant="primary" type="submit" Icon={Check}>Save</UniButton>
      <UniButton
        variant="danger"
        formaction="?/delete"
        confirmText={student.name}
        confirmTitle="Delete Student"
        Icon={Trash2}>Delete</UniButton
      >
    </div>
  </HeaderEmbellish>
  <div class="grid grid-cols-1 gap-5 md:grid-cols-4">
    <Input
      value={student.studentTelegramId}
      name="studentTelegramId"
      placeholder="@username"
      labelName="Telegram"
    ></Input>
    <input type="hidden" name="id" value={student.id} />
    <input type="hidden" name="markdown" value={markdown} />
  </div>
</form>
<Editor bind:markdownContent={markdown} />
