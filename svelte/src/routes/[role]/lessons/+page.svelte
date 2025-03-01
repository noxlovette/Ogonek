<script lang="ts">
  import { H1, Table, H2, LessonCard, UniButton } from "$lib/components";
  import { enhance } from "$app/forms";
  import { page } from "$app/state";
  import type { TableConfig, Lesson } from "$lib/types/index.js";
  import { formatDateTime } from "$lib/utils";
  import {
    user,
    searchTerm,
    pageSize,
    currentPage,
    assigneeStore,
  } from "$lib/stores";
  import { goto } from "$app/navigation";
  import { PlusCircle } from "lucide-svelte";

  let { data } = $props();
  let { students } = $derived(data);
  let { data: lessons, total } = $derived(data.lessonsPaginated);

  let role = page.params.role;
  let href = role === "t" ? "/t/lessons/l" : `/s/lessons/l`;

  const lessonConfig: TableConfig<Lesson> = {
    columns: [
      { key: "title", label: "Title" },
      { key: "topic", label: "Topic" },
      {
        key: "assigneeName",
        label: "Assignee",
        formatter: (value: string | boolean | undefined) =>
          value === $user.name ? "Not Assigned" : String(value),
      },
      {
        key: "createdAt",
        label: "Created",
        formatter: (value: string | boolean | undefined) =>
          formatDateTime(String(value)),
      },
    ],
  };

  $effect(() => {
    goto(
      `?search=${$searchTerm}&page_size=${$pageSize}&page=${$currentPage}&assignee=${$assigneeStore}`,
      {
        noScroll: true,
        keepFocus: true,
      },
    );
  });
</script>

<H1>Lessons</H1>

{#if role === "t"}
  <Table
    bind:items={data.lessonsPaginated.data}
    config={lessonConfig}
    {href}
    {total}
    {students}
  />

  <form action="?/new" method="post" use:enhance>
    {#if lessons.length === 0}
      <UniButton type="submit" variant="primary" Icon={PlusCircle}
        >Add your first one</UniButton
      >
    {/if}
  </form>
{:else}
  <section class="space-y-4">
    <H2>Recent Lessons</H2>
    <div class="grid grid-cols-1 gap-4 md:grid-cols-2">
      {#each lessons as lesson}
        <LessonCard {lesson} />
      {/each}
    </div>
  </section>
{/if}

<svelte:head>
  <title>Lessons</title>
</svelte:head>
