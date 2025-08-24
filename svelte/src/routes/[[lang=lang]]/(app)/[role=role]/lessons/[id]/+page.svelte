<script lang="ts">
  import {
    UniButton,
    LargeTitle,
    Title2,
    Title3,
    Toolbar,
    TableOfContents,
    HStack,
    VStack,
    Divider,
    Merger,
    Caption1,
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
          <UniButton Icon={Pencil} href="/t/lessons/{data.lesson.id}/edit"
            >{m.edit()}</UniButton
          >
        </Merger>
      {/if}
    </VStack>
    <VStack>
      <Badge>{formattedDate}</Badge>
      {#if role === "t"}
        <Title3>
          {data.lesson.topic}
        </Title3>

        <Caption1>
          {data.lesson.assigneeName}
        </Caption1>
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
