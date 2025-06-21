<script lang="ts">
  import type { PageData } from "./$types";
  import { LessonCard, H1, TaskCard, H3, EmptySpace } from "$lib/components";
  import { fly } from "svelte/transition";
  import type { TaskSmall, LessonSmall } from "$lib/types";
  import { getGreeting } from "$lib/utils";
  import { user } from "$lib/stores";
  import HeaderEmbellish from "$lib/components/typography/HeaderEmbellish.svelte";

  const greeting = getGreeting();

  let { data }: { data: PageData } = $props();
  let { tasks, lessons }: { tasks: TaskSmall[]; lessons: LessonSmall[] } = data;
</script>

<HeaderEmbellish>
  <H1>Good {greeting}, {$user.name}</H1>
</HeaderEmbellish>

<section class="space-y-4">
  <H3>Tasks</H3>
  <div class="grid grid-cols-1 gap-4 md:grid-cols-3">
    {#if tasks.length < 1}
      <EmptySpace>No Tasks</EmptySpace>
    {/if}
    {#each tasks as task (task.id)}
      <div transition:fly={{ y: 20, duration: 300 }}>
        <TaskCard {task} />
      </div>
    {/each}
  </div>
</section>

<section class="space-y-4">
  <H3>Lessons</H3>
  <div class="grid grid-cols-1 gap-4 md:grid-cols-3">
    {#if lessons.length < 1}
      <EmptySpace>No Lessons</EmptySpace>
    {/if}
    {#each lessons as lesson (lesson.id)}
      <div transition:fly={{ y: 20, duration: 300 }}>
        <LessonCard {lesson} />
      </div>
    {/each}
  </div>
</section>

<svelte:head>
  <title>Dashboard | Ogonek</title>
</svelte:head>
