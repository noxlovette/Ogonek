<script lang="ts">
  import { enhance } from "$app/forms";
  import {
    Editor,
    H1,
    AssigneeSelector,
    UniButton,
    HeaderEmbellish,
    Label,
  } from "$lib/components";
  import type { PageData } from "./$types";
  import { enhanceForm } from "$lib/utils";
  import { Ban, Check, Trash2 } from "lucide-svelte";
  import Input from "$lib/components/UI/forms/Input.svelte";
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
  })}
>
  <HeaderEmbellish>
    <H1>Edit Lesson</H1>
    <div class="flex items-center space-x-3">
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
  </HeaderEmbellish>

  <input type="hidden" name="id" value={lesson.id} />
  <input type="hidden" name="markdown" value={markdown} />
  <div class="grid grid-cols-1 gap-5 md:grid-cols-3">
    <Input name="title" value={lesson.title} placeholder="Title"></Input>
    <Input name="topic" value={lesson.topic} placeholder="Topic"></Input>
    <AssigneeSelector item={lesson} />
  </div>
</form>
<Editor bind:markdownContent={markdown} />
