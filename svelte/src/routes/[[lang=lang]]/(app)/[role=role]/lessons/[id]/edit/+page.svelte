<script lang="ts">
  import { enhance } from "$app/forms";
  import {
    Editor,
    LargeTitle,
    Toolbar,
    Divider,
    Merger,
    VStack,
    CancelButton,
    DeleteButton,
    SaveButton,
    PhotoPicker,
    SearchBar,
    UniButton,
    Photo,
  } from "$lib/components";
  import { enhanceForm } from "$lib/utils";
  import Input from "$lib/components/UI/forms/Input.svelte";
  import { m } from "$lib/paraglide/messages";
  import Title2 from "$lib/components/typography/Title2.svelte";
  import { , Eye, EyeClosed, ImageOff } from "lucide-svelte";
  import { invalidate } from "$app/navigation";
  let { data, form } = $props();
  let { lesson } = data;

  let markdown = $state(lesson.markdown);

  let q = "";
  let preview = $state(false);
  let showPicker = $state(true);
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

<form
  class="flex flex-col gap-4"
  action="?/unsplash"
  method="POST"
  use:enhance={enhanceForm({
    handlers: {
      success: () => {
        invalidate("edit:photo");
      },
    },
    shouldUpdate: true,
  })}
>
  <VStack>
    <Title2>Photo (Unsplash)</Title2>
    <Divider></Divider>
    <Merger>
      {#if data.lesson.photo}
        <UniButton
          type="submit"
          variant="danger"
          Icon={ImageOff}
          formaction="?/removePhoto"
        ></UniButton>
      {/if}
      {#if form?.photos}
        <UniButton
          Icon={showPicker ? Eye : EyeClosed}
          onclick={() => (showPicker = !showPicker)}
        />
      {/if}
    </Merger>

    <SearchBar bind:q placeholder="Search photos..." />
  </VStack>
  {#if showPicker}
    <PhotoPicker photos={form?.photos} chosen={lesson.photo?.id} />
  {/if}
  <Photo photo={data.lesson.photo} />
</form>

<Editor bind:markdownContent={markdown} />
