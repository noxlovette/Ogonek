<script lang="ts">
  import {
    Title1,
    UniButton,
    VStack,
    HStack,
    Toolbar,
    Table,
    Divider,
    Merger,
  } from "$lib/components";
  import { Plus } from "lucide-svelte";
  import type { PageProps } from "./$types";
  import { enhanceForm, formatDate } from "$lib/utils";
  import type { TableConfig, Content } from "$lib/types";
  import LargeTitle from "$lib/components/typography/LargeTitle.svelte";
  import { enhance } from "$app/forms";

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
  <Divider></Divider>

  <Merger>
    <form
      action="?/new"
      method="post"
      use:enhance={enhanceForm({
        messages: {
          redirect: "New content added",
        },
        navigate: true,
      })}
    >
      <UniButton Icon={Plus} type="submit">New</UniButton>
    </form>
  </Merger>
</Toolbar>

<Table config={contentConfig} items={data.content} href="content" />
