<script lang="ts">
  import {
    UniButton,
    H1,
    H2,
    H3,
    Toolbar,
    TableOfContents,
  } from "$lib/components";

  import { formatDate } from "@noxlovette/svarog";
  import type { PageData } from "./$types";
  import { Pencil } from "lucide-svelte";
  import { page } from "$app/state";
  import { m } from "$lib/paraglide/messages";

  let role = $derived(page.params.role);

  let { data }: { data: PageData } = $props();

  let formattedDate = formatDate(data.lesson.createdAt);
</script>

<Toolbar>
  <div class="flex items-baseline gap-3 md:gap-4">
    <H1>
      {#if role === "t"}
        {data.lesson.title}
      {:else}
        {data.lesson.topic}
      {/if}
    </H1>
    {#if role === "t"}
      <H3>
        {data.lesson.assigneeName}
      </H3>
    {:else}
      {formattedDate}
    {/if}
  </div>
  {#if role === "t"}
    <div class="flex items-center gap-3 md:gap-4">
      <H2>
        {data.lesson.topic}
      </H2>
      <UniButton
        Icon={Pencil}
        href="/t/lessons/{data.lesson.id}/edit"
        variant="secondary">{m.edit()}</UniButton
      >
    </div>
  {/if}
</Toolbar>
<div class="gap-4 md:grid md:grid-cols-4">
  <TableOfContents />
  <div class="markdown md:col-span-3">
    <!-- Input is sanitized with rehype -->
    {@html data.rendered}
  </div>
</div>
<svelte:head>
  <title>Lesson From {formattedDate}</title>
</svelte:head>
