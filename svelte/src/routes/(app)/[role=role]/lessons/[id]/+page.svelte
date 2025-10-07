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
    EditButton,
    Photo,
    Badge,
  } from "$lib/components";

  import { formatDateOnly } from "$lib/utils";
  import type { PageData } from "./$types";
  import { page } from "$app/state";

  let role = $derived(page.params.role);

  let { data }: { data: PageData } = $props();
  const lesson = $derived(data.lesson);

  let formattedDate = formatDateOnly(lesson.createdAt);
</script>

<Toolbar>
  <HStack>
    <VStack>
      <LargeTitle>
        {#if role === "t"}
          {lesson.title}
        {:else}
          {lesson.topic}
        {/if}
      </LargeTitle>
      <Divider />
      {#if role === "t"}
        <Merger>
          <EditButton href="/t/lessons/{lesson.id}/edit" />
        </Merger>
      {/if}
    </VStack>
    <VStack override="gap-2">
      <Badge>{formattedDate}</Badge>
      {#if role === "t"}
        {#if lesson.assignee}
          <Badge>
            {lesson.assigneeName}
          </Badge>
        {/if}
        <Title3>
          {lesson.topic}
        </Title3>
      {/if}
    </VStack>
  </HStack>
</Toolbar>
<div class="md:grid md:grid-cols-4">
  <article class="markdown md:col-span-3">
    <Photo photo={lesson.photo} />

    {@html data.rendered}
  </article>
  <TableOfContents />
</div>
