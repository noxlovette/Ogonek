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
  } from "$lib/components";

  import { formatDate } from "@noxlovette/svarog";
  import type { PageData } from "./$types";
  import { page } from "$app/state";
  import Badge from "$lib/components/cards/Badge.svelte";
  import type { PhotoURLs } from "$lib/types";

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
      </div>
    {/if}

    {@html data.rendered}
  </article>
  <TableOfContents />
</div>
<svelte:head>
  <title>Lesson From {formattedDate}</title>
</svelte:head>
