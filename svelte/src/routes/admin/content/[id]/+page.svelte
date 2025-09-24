<script lang="ts">
  import {
    LargeTitle,
    Toolbar,
    HStack,
    VStack,
    Divider,
    Merger,
    Caption1,
    EditButton,
    UniButton,
  } from "$lib/components";

  import { formatDateTime } from "$lib/utils";
  import type { PageData } from "./$types";
  import Badge from "$lib/components/cards/Badge.svelte";
  import { page } from "$app/state";
  import { ScanSearch } from "lucide-svelte";

  let { data }: { data: PageData } = $props();
</script>

<Toolbar>
  <HStack>
    <VStack>
      <LargeTitle>
        {data.content.title}
      </LargeTitle>
      <Divider />
      <Merger>
        <UniButton
          Icon={ScanSearch}
          content="Preview"
          href="/admin/content/preview/{data.content.slug}"
        ></UniButton>
        <EditButton href="{page.params.id}/edit" />
      </Merger>
    </VStack>
    <VStack>
      <Badge urgency={data.content.status == "published" ? "green" : "normal"}>
        {data.content.status}
      </Badge>
      <Caption1>
        Last edited on {formatDateTime(data.content.updatedAt)}
        {#if data.content.status == "published" && data.content.publishedAt}
          , published on {formatDateTime(data.content.publishedAt)}
        {/if}
      </Caption1>
    </VStack>
  </HStack>
</Toolbar>
<div class="markdown">
  {@html data.rendered}
</div>
<svelte:head>
  <title>Content | {data.content.title}</title>
</svelte:head>
