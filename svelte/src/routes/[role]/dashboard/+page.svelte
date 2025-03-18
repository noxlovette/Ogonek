<script lang="ts">
  import type { PageData } from "./$types";
  import { LessonCard, H1, Clock, TaskCard, H3 } from "$lib/components";
  import { fly } from "svelte/transition";
  import type { Task, Lesson } from "$lib/types";
  import { getGreeting } from "$lib/utils";
  import { user } from "$lib/stores";

  const greeting = getGreeting();

  let { data }: { data: PageData } = $props();
  let { tasks, lessons }: { tasks: Task[]; lessons: Lesson[] } = data;
</script>

<div class="space-y-4">
  <div class="flex items-center justify-between">
    <H1>{$user.name}, good {greeting}</H1>
  </div>

  <section class="space-y-4">
    <H3>Recent Tasks</H3>
    <div class="grid grid-cols-1 gap-4 md:grid-cols-3">
      {#if tasks.length < 1}
        No Tasks
      {/if}
      {#each tasks as task (task.id)}
        <div transition:fly={{ y: 20, duration: 300 }}>
          <TaskCard {task} />
        </div>
      {/each}
    </div>
  </section>

  <section class="space-y-4">
    <H3>Latest Lessons</H3>
    <div class="grid grid-cols-1 gap-4 md:grid-cols-3">
      {#if lessons.length < 1}
        No Lessons
      {/if}
      {#each lessons as lesson (lesson.id)}
        <div transition:fly={{ y: 20, duration: 300 }}>
          <LessonCard {lesson} />
        </div>
      {/each}
    </div>
  </section>
</div>

<svelte:head>
  <title>Dashboard | Ogonek</title>
</svelte:head>
