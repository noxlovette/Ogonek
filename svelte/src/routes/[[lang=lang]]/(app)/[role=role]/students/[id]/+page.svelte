<script lang="ts">
  import {
    DeckCard,
    EmptySpace,
    GridCell,
    LargeTitle,
    Title3,
    Toolbar,
    LessonCard,
    TaskCard,
    UniButton,
  } from "$lib/components";
  import Title2 from "$lib/components/typography/Title2.svelte";
  import type { PageData } from "./$types";
  import { Pencil } from "lucide-svelte";
  let { data }: { data: PageData } = $props();

  const { student, rendered, studentTasks, studentLessons, studentDecks } =
    data;
</script>

<svelte:head>
  <title>{student.name}</title>
</svelte:head>
<Toolbar>
  <div>
    <LargeTitle>{student.name}</LargeTitle>
    <Title3>
      {student.email}
    </Title3>
  </div>

  <UniButton Icon={Pencil} href="/t/students/{student.id}/edit">Edit</UniButton>
</Toolbar>
<grid class="grid gap-4 md:grid-cols-2">
  <GridCell>
    <Title2>Decks</Title2>
    {#each studentDecks as deck (deck.id)}
      <DeckCard {deck}></DeckCard>
    {/each}
    {#if studentDecks.length < 1}
      <EmptySpace>
        <Title3>No Decks</Title3>
      </EmptySpace>
    {/if}
  </GridCell>

  <GridCell>
    <Title2>Lessons</Title2>
    {#each studentLessons as lesson (lesson.id)}
      <LessonCard {lesson}></LessonCard>
    {/each}
    {#if studentLessons.length < 1}
      <EmptySpace>
        <Title3>No Lessons</Title3>
      </EmptySpace>
    {/if}
  </GridCell>

  <GridCell>
    <Title2>Tasks</Title2>
    {#each studentTasks as task (task.id)}
      <TaskCard {task}></TaskCard>
    {/each}
    {#if studentTasks.length < 1}
      <EmptySpace>
        <Title3>No Tasks</Title3>
      </EmptySpace>
    {/if}
  </GridCell>

  <GridCell>
    <Title2>Notes</Title2>
    {#if student.markdown}
      <div class="">
        {@html rendered}
      </div>
    {:else}
      <EmptySpace>
        <Title3>No Notes</Title3>
      </EmptySpace>
    {/if}
  </GridCell>
</grid>
