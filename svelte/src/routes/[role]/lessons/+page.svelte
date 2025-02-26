<script lang="ts">
  import { H1, Table, ButtonSubmit, H2, LessonCard } from "$lib/components";
  import { enhance } from "$app/forms";
  import { page } from "$app/state";
  import type { TableConfig, Lesson } from "$lib/types";
  import { formatDateTime } from "$lib/utils";
  import { user } from "$lib/stores";

  let { data } = $props();
  let { students } = $derived(data);
  let { data: lessons, total } = $derived(data.lessonsPaginated);

  let role = page.params.role;
  let href = role === "t" ? "/t/lessons/l" : `/s/lessons/l`;

  $inspect(data.lessonsPaginated.total);

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
      <ButtonSubmit buttonName="Add your first one!" />
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
