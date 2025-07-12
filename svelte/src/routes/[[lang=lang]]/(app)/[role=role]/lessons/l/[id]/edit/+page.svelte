<script lang="ts">
  import { enhance } from "$app/forms";
  import {
    Editor,
    H1,
    AssigneeSelector,
    UniButton,
    HeaderEmbellish,
  } from "$lib/components";
  import type { PageData } from "./$types";
  import { enhanceForm } from "$lib/utils";
  import { Ban, Check, Trash2 } from "lucide-svelte";
  import Input from "$lib/components/UI/forms/Input.svelte";
  import { m } from "$lib/paraglide/messages";
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
      redirect: m.changesSaved(),
      defaultError: m.failedToSaveChanges(),
    },
  })}
>
  <HeaderEmbellish>
    <H1>{m.editing()}</H1>
    <div class="flex items-center space-x-3">
      <UniButton variant="secondary" Icon={Ban} href=".">{m.cancel()}</UniButton
      >
      <UniButton variant="primary" type="submit" Icon={Check}
        >{m.save()}</UniButton
      >
      <UniButton
        variant="danger"
        formaction="?/delete"
        Icon={Trash2}
        confirmText={lesson.title}
        confirmTitle="Delete Lesson">{m.delete()}</UniButton
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
