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
    Toggler,
  } from "$lib/components";
  import { enhanceForm } from "$lib/utils";
  import Input from "$lib/components/UI/forms/Input.svelte";
  import { m } from "$lib/paraglide/messages";
  import Title2 from "$lib/components/typography/Title2.svelte";
  import { Eye, EyeClosed, ImageOff } from "@lucide/svelte";
  import { invalidate } from "$app/navigation";
  let { data, form } = $props();
  let { lesson } = data;

  let markdown = $state(lesson.markdown);

  let showPicker = $state(true);

  let q = "";

  let assigned = $state(lesson.assignee ? true : false);
</script>

<form
  method="POST"
  action="?/update"
  class="gap-default mb-4 flex flex-col"
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
        <DeleteButton />
      </Merger>
      <Merger>
        <SaveButton />
      </Merger>
    </VStack>
  </Toolbar>

  <input type="hidden" name="markdown" value={markdown} />
  <div class="gap-default grid grid-cols-1 md:grid-cols-3">
    <Input
      name="title"
      labelName="Название"
      invalid={form?.title}
      invalidDescription="Как назовем? Иосиф?"
      value={lesson.title}
      placeholder="Title"
    ></Input>
    <Input
      name="topic"
      invalid={form?.topic}
      invalidDescription="О чем говорили?"
      labelName="Тема"
      value={lesson.topic}
      placeholder="Topic"
    ></Input>

    {#if assigned}
      <Input
        name="assignee"
        labelName="Назначено"
        invalid={form?.assignee}
        invalidDescription="С кем было занятие-то?"
        item={lesson}
        type="assignee"
        placeholder="С кем было занятие"
      />
    {/if}
  </div>
  <Toggler
    bind:value={assigned}
    name="assigned"
    title={assigned
      ? "Это занятие будет привязано к ученику"
      : "Это занятие ни к кому не будет привязано"}
  />
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
          content="Убрать фотографию"
          formaction="?/removePhoto"
        ></UniButton>
      {/if}
      {#if form?.photos}
        <UniButton
          content="Показать/скрыть фотографии"
          Icon={showPicker ? Eye : EyeClosed}
          onclick={() => (showPicker = !showPicker)}
        />
      {/if}
    </Merger>

    <SearchBar bind:q placeholder="Search photos..." />
  </VStack>
  {#if showPicker}
    <PhotoPicker
      photos={form?.photos}
      error={form?.unsplashError}
      chosen={lesson.photo?.id}
    />
  {/if}
  <Photo photo={data.lesson.photo} />
</form>

<Editor bind:markdownContent={markdown} />
