<script lang="ts">
  import type { PageData } from "./$types";
  import { LessonCard, H1, Clock, TaskCard, H3 } from "$lib/components";
  import { fly } from "svelte/transition";
  import type { Task, Lesson } from "$lib/types";

  let { data }: { data: PageData } = $props();
  let { tasks, lessons }: { tasks: Task[]; lessons: Lesson[] } = data;

  let pending = tasks.filter((task) => !task.completed);
</script>

<div class="space-y-4">
  <div class="flex items-center justify-between">
    <H1>Dashboard</H1>
    <Clock />
  </div>

  <section class="space-y-4">
    <H3>
      Recent Tasks
    </H3>
    <div class="grid grid-cols-1 gap-4 md:grid-cols-3">
      {#if pending.length < 1}
        No Tasks
      {/if}
      {#each pending.slice(0, 6) as task (task.id)}
        <div transition:fly={{ y: 20, duration: 300 }}>
          <TaskCard {task} />
        </div>
      {/each}
    </div>
  </section>

  <section class="space-y-4">
    <H3>
      Latest Lessons
    </H3>
    <div class="grid grid-cols-1 gap-4 md:grid-cols-3">
      {#if lessons.length < 1}
        No Lessons
      {/if}
      {#each lessons.slice(0, 6) as lesson (lesson.id)}
        <div transition:fly={{ y: 20, duration: 300 }}>
          <LessonCard {lesson} />
        </div>
      {/each}
    </div>
  </section>
</div>

<svelte:head>
  <title>Dashboard | Firelight</title>
</svelte:head>
