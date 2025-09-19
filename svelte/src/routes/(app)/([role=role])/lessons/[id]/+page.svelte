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
  } from "$lib/components";

  import { formatDate } from "$lib/utils";
  import type { PageData } from "./$types";
  import { page } from "$app/state";
  import Badge from "$lib/components/cards/Badge.svelte";

  let role = $derived(page.params.role);

  let { data }: { data: PageData } = $props();

  let formattedDate = formatDate(data.lesson.createdAt);
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
    <VStack styling="">
      {#if role === "t"}
        <Title3>
          {data.lesson.topic}
        </Title3>

        <Caption1>
          {data.lesson.assigneeName}
        </Caption1>
      {/if}
      <Badge>{formattedDate}</Badge>
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
