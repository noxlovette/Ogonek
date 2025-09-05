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
  } from "$lib/components";
  import { enhanceForm } from "$lib/utils";
  import Input from "$lib/components/UI/forms/Input.svelte";
  import { m } from "$lib/paraglide/messages";
  import Title2 from "$lib/components/typography/Title2.svelte";
  import { ChartNoAxesGantt, Eye, ImageOff } from "lucide-svelte";
  import type { PhotoURLs } from "$lib/types";
  import { invalidate } from "$app/navigation";
  let { data, form } = $props();
  let { lesson } = data;

  let markdown = $state(lesson.markdown);

  let q = "";
  let preview = $state(false);
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
    <Title2>Photo</Title2>
    <Divider></Divider>
    {#if data.lesson.photo}
      <Merger>
        <UniButton type="submit" Icon={ImageOff} formaction="?/removePhoto"
        ></UniButton>
      </Merger>
    {/if}
    <SearchBar bind:q placeholder="Search photos..." />
  </VStack>
  <PhotoPicker photos={form?.photos} chosen={lesson.photo?.id} />
  {#if data.lesson.photo}
    <div class="relative h-30 w-full overflow-hidden rounded-t-xl">
      <div
        class="absolute inset-0 z-10 bg-cover bg-center"
        style="background-image: url('{(data.lesson.photo?.urls as PhotoURLs)
          .small}')"
      ></div>

      <div
        class="absolute inset-0 z-20 bg-cover bg-center"
        style="background-image: url('{(data.lesson.photo?.urls as PhotoURLs)
          .full}')"
      ></div>

      <img
        src={(data.lesson.photo?.urls as PhotoURLs).full}
        alt={data.lesson.photo?.altDescription}
        loading="lazy"
        class="absolute inset-0 -z-10 h-0 w-0 opacity-0"
      />

      {#if data.lesson.photo?.photographerName}
        <div
          class="absolute right-0 bottom-0 z-30 m-2 rounded-full bg-stone-100 px-2 py-1 text-xs dark:bg-stone-800"
        >
          Photo by <a
            href={`https://unsplash.com/@${data.lesson.photo.photographerUsername}`}
            target="_blank"
            rel="noopener noreferrer"
            aria-label="Visit photographer's Unsplash profile (opens in new tab)"
            >{data.lesson.photo.photographerName}</a
          >
        </div>
      {/if}
    </div>
  {/if}
</form>

<VStack>
  <Title2>Markdown</Title2>
  <Divider></Divider>
  <Merger>
    <UniButton
      variant={preview ? "primary" : "prominent"}
      Icon={ChartNoAxesGantt}
      onclick={() => (preview = false)}>Edit</UniButton
    >
    <UniButton
      variant={preview ? "prominent" : "primary"}
      Icon={Eye}
      onclick={() => (preview = true)}>Preview</UniButton
    >
  </Merger>
</VStack>
<Editor bind:markdownContent={markdown} {preview} />
