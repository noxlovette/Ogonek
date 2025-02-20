<script lang="ts">
  import {
    Dashboard,
    Todo,
    Lessons,
    Quizlet,
    Settings,
    Students,
    Zoom,
    Sidebar,
    WorkArea,
    BottomMenu,
    UsefulLinks,
    WordOfTheDay,
  } from "$lib/components";
  import { lessonStore, studentStore, taskStore } from "$lib/stores";

  import { page } from "$app/state";
  import { setContext } from "svelte";
  import type { Word } from "$lib/types";

  let { data, children } = $props();
  const role = $derived(page.params.role);

  let elements = $state([Dashboard, Todo, Lessons, Zoom, Quizlet, Settings]);

  if (role === "t") {
    elements = [Dashboard, Todo, Lessons, Students, Quizlet, Settings];
  }

  const { tasks, lessons, students, word } = data;

  lessonStore.setLessons(lessons);
  taskStore.setTasks(tasks);
  studentStore.setStudents(students);
  setContext<Promise<Word>>("word", word);
</script>

<Sidebar class="text-cacao-50" {elements} />
<WorkArea>
  {@render children?.()}
</WorkArea>
{#if role !== "t"}
  <Sidebar
    class="dark:bg-milk-950 bg-inherit text-inherit shadow-none ring-transparent dark:ring-transparent"
    subclass="divide-transparent dark:divide-transparent"
    elements={[UsefulLinks, WordOfTheDay]}
  ></Sidebar>
{/if}
<BottomMenu {elements} />

<svelte:head>
  <title>Tasks</title>
</svelte:head>
