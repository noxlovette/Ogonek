<script lang="ts">
  import { H1, H2 } from "$lib/components";
  import { user } from "$lib/stores";
  import { formatDateTime } from "$lib/utils";
  import type { PageData } from "./$types";
  import { goto } from "$app/navigation";
  import { page } from "$app/state";

  let { data }: { data: PageData } = $props();

  let formattedDate = formatDateTime(data.lesson.createdAt);
</script>

{#if page.params.role === "t"}
  <div class="flex items-baseline space-x-4">
    <H1>{data.lesson.title}</H1>
    <button
      onclick={() =>
        goto(`/t/lessons/l/${data.lesson.id}/edit`, { replaceState: true })}
      class="bg-cacao-600 text-cacao-50 hover:bg-cacao-700 focus:ring-cacao-500 rounded-lg px-4 py-2 transition-colors focus:ring-2 focus:ring-offset-2 focus:outline-none disabled:opacity-50"
      >Edit</button
    >
  </div>
  <div class="flex space-x-4">
    <div class="space-y-2">
      <p class="text-milk-700 block font-medium">Topic</p>
      <h3 class="min-w-48">
        {data.lesson.topic}
      </h3>
    </div>
    <div class="space-y-2">
      <p class="text-milk-700 block font-medium">Student</p>
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
