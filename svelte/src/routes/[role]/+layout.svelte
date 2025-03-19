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
    RecentLessons,
  } from "$lib/components";
  import { lessonStore, studentStore, taskStore } from "$lib/stores";

  import { page } from "$app/state";
  import { setContext } from "svelte";
  import type { Word } from "$lib/types/index.js";

  let { data, children } = $props();
  const role = page.params.role;

  let elements = $state([Dashboard, Todo, Lessons, Words, Zoom, Settings]);

  if (role === "t") {
    elements = [Dashboard, Todo, Lessons, Students, Words, Settings];
  }

  lessonStore.setLessons(data.lessons);
  taskStore.setTasks(data.tasks);
  studentStore.setStudents(data.students);
  setContext<Promise<Word>>("word", data.word);
</script>

<Sidebar {elements} />
<WorkArea>
  {@render children?.()}
</WorkArea>
{#if role !== "t"}
  <Rightbar elements={[UsefulLinks, WordOfTheDay, RecentLessons]}></Rightbar>
{/if}
<BottomMenu {elements} />

<svelte:head>
  <title>Tasks</title>
</svelte:head>
