<script lang="ts">
  import {
    Dashboard,
    Todo,
    Lessons,
    Words,
    Settings,
    Students,
    Zoom,
    Sidebar,
    WorkArea,
    BottomMenu,
    UsefulLinks,
    WordOfTheDay,
    Rightbar,
  } from "$lib/components";
  import { lessonStore, studentStore, taskStore } from "$lib/stores";

  import { page } from "$app/state";
  import { setContext } from "svelte";
  import type { Word } from "$lib/types";

  let { data, children } = $props();
  const role = page.params.role;

  let elements = $state([Dashboard, Todo, Lessons, Zoom, Words, Settings]);

  if (role === "t") {
    elements = [Dashboard, Todo, Lessons, Students, Words, Settings];
  }

  const { tasks, lessons, students, word } = data;

  lessonStore.setLessons(lessons);
  taskStore.setTasks(tasks);
  studentStore.setStudents(students);
  setContext<Promise<Word>>("word", word);
</script>

<Sidebar {elements} />
<WorkArea>
  {@render children?.()}
</WorkArea>
{#if role !== "t"}
  <Rightbar elements={[UsefulLinks, WordOfTheDay]}></Rightbar>
{/if}
<BottomMenu {elements} />

<svelte:head>
  <title>Tasks</title>
</svelte:head>
