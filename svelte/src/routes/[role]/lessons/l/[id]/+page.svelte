<script lang="ts">
  import { UniButton, H1, H2, H3, HeaderEmbellish } from "$lib/components";

  import { formatDate } from "@noxlovette/svarog";
  import type { PageData } from "./$types";
  import { Pencil } from "lucide-svelte";
  import { page } from "$app/state";

  let role = $derived(page.params.role);

  let { data }: { data: PageData } = $props();

  let formattedDate = formatDate(data.lesson.createdAt);
</script>

<HeaderEmbellish>
  <div>
    <H1>
      {#if role === "t"}
        {data.lesson.title}
      {:else}
        Lesson From {formattedDate}
      {/if}
    </H1>
    {#if role === "t"}
      <H3>
        {data.lesson.assigneeName}
      </H3>
    {/if}
  </div>
  <div class="flex items-center space-x-3">
    {#if role === "t"}
      <UniButton
        Icon={Pencil}
        href="/t/lessons/l/{data.lesson.id}/edit"
        variant="outline">Edit</UniButton
      >
    {/if}
    <H2>
      {data.lesson.topic}
    </H2>
  </div>
</HeaderEmbellish>
<div class="markdown">
  <!-- Input is sanitized with rehype -->
  {@html data.rendered}
</div>

<svelte:head>
  <title>Lesson From {formattedDate}</title>
</svelte:head>
