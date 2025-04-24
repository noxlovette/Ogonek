<script lang="ts">
  import {
    DeckCard,
    EmptySpace,
    GridCell,
    H1,
    H3,
    HeaderEmbellish,
    LessonCard,
    TaskCard,
    UniButton,
  } from "$lib/components";
  import H2 from "$lib/components/typography/H2.svelte";
  import type { PageData } from "./$types";
  import { Pencil } from "lucide-svelte";
  let { data }: { data: PageData } = $props();

  const { student, rendered, studentTasks, studentLessons, studentDecks } =
    data;
</script>

<svelte:head>
  <title>{student.name}</title>
</svelte:head>
<HeaderEmbellish>
  <div>
    <H1>{student.name}</H1>
    <H3>
      {student.email}
    </H3>
  </div>

  <UniButton
    Icon={Pencil}
    href="/t/students/st/{student.id}/edit"
    variant="outline">Edit</UniButton
  >
</HeaderEmbellish>
<grid class="grid gap-4 md:grid-cols-2">
  <GridCell>
    <H2>Decks</H2>
    {#each studentDecks as deck}
      <DeckCard {deck}></DeckCard>
    {/each}
    {#if studentDecks.length < 1}
      <EmptySpace>
        <H3>No Decks</H3>
      </EmptySpace>
    {/if}
  </GridCell>

  <GridCell>
    <H2>Lessons</H2>
    {#each studentLessons as lesson}
      <LessonCard {lesson}></LessonCard>
    {/each}
    {#if studentLessons.length < 1}
      <EmptySpace>
        <H3>No Lessons</H3>
      </EmptySpace>
    {/if}
  </GridCell>

  <GridCell>
    <H2>Tasks</H2>
    {#each studentTasks as task}
      <TaskCard {task}></TaskCard>
    {/each}
    {#if studentTasks.length < 1}
      <EmptySpace>
        <H3>No Tasks</H3>
      </EmptySpace>
    {/if}
  </GridCell>

  <GridCell>
    <H2>Notes</H2>
    {#if student.markdown}
      <div class="">
        {@html rendered}
      </div>
    {:else}
      <EmptySpace>
        <H3>No Notes</H3>
      </EmptySpace>
    {/if}
  </GridCell>
</grid>
