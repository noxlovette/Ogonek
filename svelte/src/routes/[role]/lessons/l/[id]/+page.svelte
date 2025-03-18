<script lang="ts">
  import { UniButton, H1, H2 } from "$lib/components";
  import { user } from "$lib/stores";

  import { formatDate } from "@noxlovette/svarog";
  import type { PageData } from "./$types";
  import { Pencil } from "lucide-svelte";
  import { page } from "$app/state";

  let { data }: { data: PageData } = $props();

  let formattedDate = formatDate(data.lesson.createdAt);
</script>

{#if page.params.role === "t"}
  <div class="flex items-baseline space-x-4">
    <H1>{data.lesson.title}</H1>
    <UniButton
      Icon={Pencil}
      href="/t/lessons/l/{data.lesson.id}/edit"
      variant="outline">Edit</UniButton
    >
  </div>
  <div class="flex space-x-4">
    <div class="space-y-2">
      <p class="block font-medium text-stone-700">Topic</p>
      <h3 class="min-w-48">
        {data.lesson.topic}
      </h3>
    </div>
    <div class="space-y-2">
      <p class="block font-medium text-stone-700">Student</p>
      <h3 class="min-w-48">
        {#if data.lesson.assigneeName === $user.username}
          Not Assigned
        {:else}
          {data.lesson.assigneeName}
        {/if}
      </h3>
    </div>
  </div>
  <div class="markdown">
    {@html data.rendered}
  </div>
{:else}
  <div class="flex items-baseline space-x-4">
    <H1>Lesson From {formattedDate}</H1>
  </div>
  <div class="">
    <H2>
      {data.lesson.topic}
    </H2>
  </div>
  <div class="markdown">
    {@html data.rendered}
  </div>
{/if}

<svelte:head>
  <title>Lesson From {formattedDate}</title>
</svelte:head>
