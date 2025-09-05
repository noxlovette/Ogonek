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
    Grid,
    Callout,
  } from "$lib/components";
  import { enhanceForm } from "$lib/utils";
  import Input from "$lib/components/UI/forms/Input.svelte";
  import { m } from "$lib/paraglide/messages";
  import Title2 from "$lib/components/typography/Title2.svelte";
  let { data, form } = $props();
  let { lesson } = data;

  let markdown = $state(lesson.markdown);

  let q = "";
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

{#if lesson.photo}
  <div class="mb-6 rounded-xl border border-gray-200 bg-gray-50 p-4">
    <div class="flex items-start gap-4">
      <img
        src={lesson.photo.urls?.small || lesson.photo.urls}
        alt={lesson.photo.altDescription || "Lesson photo"}
        class="h-24 w-24 flex-shrink-0 rounded-lg object-cover"
      />
      <div class="min-w-0 flex-1">
        <p class="mb-1 text-sm font-medium text-gray-900">
          Current lesson photo
        </p>
        <p class="truncate text-xs text-gray-500">
          {lesson.photo.altDescription || "No description available"}
        </p>
        {#if lesson.photo.photographerName}
          <p class="mt-1 text-xs text-gray-500">
            Photo by {lesson.photo.photographerName}
          </p>
        {/if}
      </div>
    </div>
  </div>
{/if}

<Editor bind:markdownContent={markdown} />

<VStack>
  <Title2>Unsplash Photos</Title2>
  <Divider></Divider>

  {#if q}
    <Callout>
      Showing results for {q}
    </Callout>
  {/if}
</VStack>
<Grid>
  <form
    class=""
    action="?/unsplash"
    method="POST"
    use:enhance={enhanceForm({
      messages: {
        failure: "Failed to add photo",
        success: "Photo updated",
      },
    })}
  >
    <SearchBar bind:q />
  </form>

  {#if form?.photo}
    Failed to save photo
  {/if}

  <PhotoPicker photos={form?.photos} chosen={lesson.photo?.unsplashId} />
</Grid>
