<script lang="ts">
  /**
   * @component
   * @description Searches for tasks, lessons, and lives in the header
   */

  import type { Lesson, Task } from "$lib/types";
  import { lessonStore, taskStore } from "$lib/stores";
  import SearchBar from "./SearchBar.svelte";
  import LessonCard from "$lib/components/cards/LessonCard.svelte";
  import TaskCard from "$lib/components/cards/TaskCard.svelte";

  let query = $state("");
  let filteredLessons: Lesson[] = $state([]);
  let filteredTasks: Task[] = $state([]);

  $effect(() => {
    const lowercaseQuery = query.toLowerCase();

    filteredLessons = $lessonStore.filter((lesson) =>
      [lesson.title, lesson.topic, lesson.assigneeName, lesson.markdown].some(
        (field) => field.toLowerCase().includes(lowercaseQuery),
      ),
    );

    filteredTasks = $taskStore.filter((task) =>
      [task.title, task.markdown].some((field) =>
        field.toLowerCase().includes(lowercaseQuery),
      ),
    );
  });
</script>

<div class="z-40">
  <SearchBar bind:query />

  <div
    class="bg-cacao-50 absolute top-20 left-1/2 w-11/12 max-w-2xl -translate-x-1/2 rounded-lg shadow-xl md:w-full dark:bg-stone-900"
  >
    {#if query}
      <div class="max-h-[32rem] divide-y divide-stone-200 overflow-y-auto">
        {#if filteredLessons.length}
          <section class="p-4">
            <h2 class="mb-3 text-sm font-medium text-stone-500">Lessons</h2>
            <div class="space-y-2">
              {#each filteredLessons as lesson}
                <LessonCard {lesson} />
              {/each}
            </div>
          </section>
        {/if}

        {#if filteredTasks.length}
          <section class="p-4">
            <h2 class="mb-3 text-sm font-medium text-stone-500">Tasks</h2>
            <div class="space-y-2">
              {#each filteredTasks as task}
                <TaskCard interactive={false} {task} />
              {/each}
            </div>
          </section>
        {/if}

        {#if !filteredLessons.length && !filteredTasks.length}
          <div class="p-8 text-center text-stone-500">No results found</div>
        {/if}
      </div>
    {/if}
  </div>
</div>
