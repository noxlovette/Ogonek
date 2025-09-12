<script lang="ts">
  import {
    Title1,
    UniButton,
    VStack,
    HStack,
    Toolbar,
    Table,
  } from "$lib/components";
  import { Plus, Eye, Edit, Pencil } from "lucide-svelte";
  import type { PageProps } from "./$types";
  import type { components } from "$lib/types/api/gen/openapi";
  import { formatDate } from "$lib/utils";
  import Badge from "$lib/components/cards/Badge.svelte";
  import type { TableConfig, Content } from "$lib/types";
  import LargeTitle from "$lib/components/typography/LargeTitle.svelte";

  let { data }: PageProps = $props();

  const contentConfig: TableConfig<Content> = {
    columns: [
      { key: "title", label: "Title" },
      { key: "slug", label: "Slug" },
      { key: "status", label: "Status" },
      {
        key: "updatedAt",
        label: "Updated",
        formatter: (value: string | number | undefined | null) =>
          formatDate(String(value)),
      },
    ],
  };
</script>

<Toolbar>
  <LargeTitle>Content</LargeTitle>
</Toolbar>

<Table config={contentConfig} items={data.content} href="content" />
