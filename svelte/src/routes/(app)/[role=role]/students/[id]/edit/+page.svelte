<script lang="ts">
  import { enhance } from "$app/forms";
  import {
    Editor,
    LargeTitle,
    Toolbar,
    UniButton,
    Input,
    SaveButton,
    CancelButton,
    DeleteButton,
  } from "$lib/components";
  import type { Student } from "$lib/types";
  import { Ban, Check, Trash2 } from "lucide-svelte";
  import type { PageData } from "./$types";
  import { enhanceForm } from "$lib/utils";
  import VStack from "$lib/components/UI/layout/VStack.svelte";

  let { data }: { data: PageData } = $props();
  let { student }: { student: Student } = data;
  let markdown = $state(student.markdown || "");
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
  <Toolbar>
    <LargeTitle>{student.name}</LargeTitle>
    <VStack>
      <CancelButton />
      <SaveButton></SaveButton>
      <DeleteButton />
    </VStack>
  </Toolbar>
  <div class="grid grid-cols-1 gap-5 md:grid-cols-4">
    <Input
      value={student.studentTelegramId}
      name="studentTelegramId"
      placeholder="@username"
      labelName="Телеграм"
    ></Input>
    <input type="hidden" name="id" value={student.id} />
    <input type="hidden" name="markdown" value={markdown} />
  </div>
</form>
<Editor bind:markdownContent={markdown} />
