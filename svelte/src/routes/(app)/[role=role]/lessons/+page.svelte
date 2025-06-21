<script lang="ts">
  import {
    H1,
    Table,
    LessonCard,
    UniButton,
    HeaderEmbellish,
    EmptySpace,
    H3,
  } from "$lib/components";
  import { enhance } from "$app/forms";
  import { enhanceForm } from "$lib/utils";
  import { page } from "$app/state";
  import type { TableConfig, Lesson } from "$lib/types/index.js";
  import { formatDate } from "@noxlovette/svarog";

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
          formatDate(String(value)),
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

<HeaderEmbellish>
  <H1>Lessons</H1>
  {#if role === "t"}
    <form
      action="?/new"
      method="post"
      use:enhance={enhanceForm({
        messages: {
          redirect: "New Lesson Created",
        },
        navigate: true,
      })}
    >
      <UniButton Icon={PlusCircle} type="submit" variant="primary"
        >New</UniButton
      >
    </form>
  {/if}
</HeaderEmbellish>
{#if role === "t"}
  <Table
    bind:items={data.lessonsPaginated.data}
    config={lessonConfig}
    {href}
    {total}
    {students}
  />
{:else}
  {#if lessons.length < 1}
    <EmptySpace>
      <H3>No Lessons</H3>
    </EmptySpace>
  {/if}
  <section class="space-y-4">
    <div class="grid grid-cols-1 gap-4 md:grid-cols-3">
      {#each lessons as lesson (lesson.id)}
        <LessonCard {lesson} />
      {/each}
    </div>
  </section>
{/if}

<svelte:head>
  <title>Lessons</title>
</svelte:head>
