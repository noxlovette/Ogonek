<script lang="ts">
  import {
    LargeTitle,
    Title3,
    Toolbar,
    TableOfContents,
    HStack,
    VStack,
    Divider,
    Merger,
    Caption1,
    EditButton,
    Photo,
    Badge,
  } from "$lib/components";

  import { formatDateOnly } from "$lib/utils";
  import type { PageData } from "./$types";
  import { page } from "$app/state";

  let role = $derived(page.params.role);

  let { data }: { data: PageData } = $props();

  let formattedDate = formatDateOnly(data.lesson.createdAt);
</script>

<Toolbar>
  <HStack>
    <VStack>
      <LargeTitle>
        {#if role === "t"}
          {data.lesson.title}
        {:else}
          {data.lesson.topic}
        {/if}
      </LargeTitle>
      <Divider />
      {#if role === "t"}
        <Merger>
          <EditButton href="/t/lessons/{data.lesson.id}/edit" />
        </Merger>
      {/if}
    </VStack>
    <VStack override="gap-2">
      <Badge>{formattedDate}</Badge>
      {#if role === "t"}
        <Badge>
          {data.lesson.assigneeName}
        </Badge>
        <Title3>
          {data.lesson.topic}
        </Title3>
      {/if}
    </VStack>
  </HStack>
</Toolbar>
<div class="md:grid md:grid-cols-4">
  <article class="markdown md:col-span-3">
    <Photo photo={data.lesson.photo} />

    {@html data.rendered}
  </article>
  <TableOfContents />
</div>
