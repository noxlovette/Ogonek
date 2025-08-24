<script lang="ts">
  import {
    UniButton,
    H1,
    H2,
    H3,
    Toolbar,
    TableOfContents,
    HStack,
    VStack,
    Divider,
    Merger,
    Caption,
  } from "$lib/components";

  import { formatDate } from "@noxlovette/svarog";
  import type { PageData } from "./$types";
  import { Pencil } from "lucide-svelte";
  import { page } from "$app/state";
  import { m } from "$lib/paraglide/messages";
  import Badge from "$lib/components/cards/Badge.svelte";

  let role = $derived(page.params.role);

  let { data }: { data: PageData } = $props();

  let formattedDate = formatDate(data.lesson.createdAt);
</script>

<Toolbar>
  <HStack>
    <VStack>
      <H1>
        {#if role === "t"}
          {data.lesson.title}
        {:else}
          {data.lesson.topic}
        {/if}
      </H1>
      <Divider />
      {#if role === "t"}
        <Merger>
          <UniButton Icon={Pencil} href="/t/lessons/{data.lesson.id}/edit"
            >{m.edit()}</UniButton
          >
        </Merger>
      {/if}
    </VStack>
    <VStack>
      <Badge badgeText={formattedDate} />
      {#if role === "t"}
        <H3>
          {data.lesson.topic}
        </H3>

        <Caption>
          {data.lesson.assigneeName}
        </Caption>
      {/if}
    </VStack>
  </HStack>
</Toolbar>
<div class="gap-4 md:grid md:grid-cols-4">
  <div class="markdown md:col-span-3">
    <!-- Input is sanitized with rehype -->
    {@html data.rendered}
  </div>
  <TableOfContents />
</div>
<svelte:head>
  <title>Lesson From {formattedDate}</title>
</svelte:head>
