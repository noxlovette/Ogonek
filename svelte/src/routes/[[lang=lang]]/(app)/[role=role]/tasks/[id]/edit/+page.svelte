<script lang="ts">
  import { enhance } from "$app/forms";

  import { Ban, Trash2, Check } from "lucide-svelte";
  import { enhanceForm } from "$lib/utils";
  import {
    Editor,
    LargeTitle,
    UniButton,
    Toolbar,
    Multipart,
    Input,
    Merger,
    Divider,
  } from "$lib/components";
  import type { PageData } from "./$types";
  import PrioritySlider from "$lib/components/UI/forms/PrioritySlider.svelte";
  import { m } from "$lib/paraglide/messages";
  import VStack from "$lib/components/UI/toolbar/VStack.svelte";

  let { data }: { data: PageData } = $props();
  let { task, files } = data;

  let markdown = $state(task.markdown);

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
  class="mb-4 space-y-6"
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
        <UniButton Icon={Ban} href=".">{m.cancel()}</UniButton>
        <UniButton
          variant="danger"
          Icon={Trash2}
          formaction="?/delete"
          confirmText={task.title}
          confirmTitle="Delete Task">{m.delete()}</UniButton
        >
      </Merger>
      <Merger>
        <UniButton variant="prominent" type="submit" Icon={Check}
          >{m.save()}</UniButton
        >
      </Merger>
    </VStack>
  </Toolbar>

  <input type="hidden" name="markdown" value={markdown} />

  <div class="grid grid-cols-1 gap-5 md:grid-cols-4">
    <Input name="title" value={task.title} placeholder="Title"></Input>
    <Input name="assignee" type="assignee" item={task} />

    <Input
      bind:value={dueDate}
      type="date"
      name="dueDate"
      labelName="Due Date"
      placeholder="Due Date"
    ></Input>
    <div class="mt-2 self-end">
      <PrioritySlider priority={task.priority} />
    </div>
  </div>
</form>

<div class="grid grid-cols-3 gap-4 space-x-4">
  <Editor bind:markdownContent={markdown} />
  <Multipart taskId={task.id} />
</div>
