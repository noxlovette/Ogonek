<script lang="ts">
  import { enhance } from "$app/forms";
  import {
    Editor,
    LargeTitle,
    UniButton,
    Toolbar,
    Divider,
    Merger,
    VStack,
    CancelButton,
    DeleteButton,
    SaveButton,
  } from "$lib/components";
  import { enhanceForm } from "$lib/utils";
  import Input from "$lib/components/UI/forms/Input.svelte";
  import { m } from "$lib/paraglide/messages";
  let { data, form } = $props();
  let { lesson } = data;

  let markdown = $state(lesson.markdown);
</script>

<form
  method="POST"
  action="?/update"
  class="gap-4"
  use:enhance={enhanceForm({
    messages: {
      redirect: m.changesSaved(),
      defaultError: m.failedToSaveChanges(),
    },
  })}
>
  <Toolbar>
    <LargeTitle>{m.editing()}</LargeTitle>
    <Divider />
    <VStack>
      <Merger>
        <CancelButton />
        <DeleteButton confirmText={lesson.title} confirmTitle="Delete Lesson" />
      </Merger>
      <Merger>
        <SaveButton />
      </Merger>
    </VStack>
  </Toolbar>

  <input type="hidden" name="id" value={lesson.id} />
  <input type="hidden" name="markdown" value={markdown} />
  <div class="grid grid-cols-1 gap-5 md:grid-cols-3">
    <Input name="title" value={lesson.title} placeholder="Title"></Input>
    <Input name="topic" value={lesson.topic} placeholder="Topic"></Input>
    <Input name="assignee" item={lesson} type="assignee" />
  </div>
</form>
<Editor bind:markdownContent={markdown} />
