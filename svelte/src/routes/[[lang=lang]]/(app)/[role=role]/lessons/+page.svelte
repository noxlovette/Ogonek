<script lang="ts">
  import {
    H1,
    Table,
    LessonCard,
    UniButton,
    HeaderEmbellish,
    EmptySpace,
    H3,
    SearchBar,
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
  import { m } from "$lib/paraglide/messages.js";

  let { data } = $props();
  let { students } = $derived(data);
  let { data: lessons, total } = $derived(data.lessonsPaginated);

  let role = page.params.role;
  let href = role === "t" ? "/t/lessons/l" : `/s/lessons/l`;

  const lessonConfig: TableConfig<Lesson> = {
    columns: [
      { key: "title", label: m.title() },
      { key: "topic", label: m.topic() },
      {
        key: "assigneeName",
        label: m.assignee(),
        formatter: (value: string | boolean | undefined) =>
          value === $user.name ? m.notAssigned() : String(value),
      },
      {
        key: "createdAt",
        label: m.created(),
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
  <div class="flex flex-col gap-3 md:flex-row md:gap-4">
    <H1>{m.lessons()}</H1>
    {#if role === "t"}
      <form
        action="?/new"
        method="post"
        use:enhance={enhanceForm({
          messages: {
            redirect: m.newLessonCreated(),
          },
          navigate: true,
        })}
      >
        <UniButton Icon={PlusCircle} type="submit" variant="primary"
          >{m.new()}</UniButton
        >
      </form>
    {/if}
  </div>
</HeaderEmbellish>
{#if role === "t"}
  <Table
    items={data.lessonsPaginated.data}
    config={lessonConfig}
    {href}
    {total}
    {students}
  />
{:else}
  <SearchBar />
  {#if lessons.length < 1}
    <EmptySpace>
      <H3>{m.noLessons}</H3>
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
  <title>{m.lessons()}</title>
</svelte:head>
