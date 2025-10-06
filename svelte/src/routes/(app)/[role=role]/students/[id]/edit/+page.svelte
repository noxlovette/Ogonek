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
  import { enhanceForm } from "$lib/utils";
  import VStack from "$lib/components/UI/layout/VStack.svelte";
  import texts from "$lib/texts.js";

  let { data } = $props();
  let { student } = data;
  let markdown = $state(student.markdown || "");
</script>

<form
  method="POST"
  action="?/update"
  class="mb-4 space-y-4"
  use:enhance={enhanceForm({
    messages: {
      redirect: texts.crud.updated,
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
