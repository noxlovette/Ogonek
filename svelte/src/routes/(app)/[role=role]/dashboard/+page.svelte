<script lang="ts">
  import type { PageData } from "./$types";
  import {
    LessonCard,
    H1,
    TaskCard,
    H3,
    EmptySpace,
    DeckCard,
  } from "$lib/components";
  import { fly } from "svelte/transition";
  import { getGreeting } from "$lib/utils";
  import { user } from "$lib/stores";
  import HeaderEmbellish from "$lib/components/typography/HeaderEmbellish.svelte";

  const greeting = getGreeting();

  let { data }: { data: PageData } = $props();
</script>

<HeaderEmbellish>
  <H1>Good {greeting}, {$user.name}</H1>
</HeaderEmbellish>

<section class="space-y-4">
  <H3>Tasks</H3>
  <div class="grid grid-cols-1 gap-4 md:grid-cols-3">
    {#if data.tasks.data.length < 1}
      <EmptySpace>No Tasks</EmptySpace>
    {/if}
    {#each data.tasks.data as task (task.id)}
      <div transition:fly={{ y: 20, duration: 300 }}>
        <TaskCard {task} />
      </div>
    {/each}
  </div>
</section>

<section class="space-y-4">
  <H3>Lessons</H3>
  <div class="grid grid-cols-1 gap-4 md:grid-cols-3">
    {#if data.lessons.data.length < 1}
      <EmptySpace>No Lessons</EmptySpace>
    {/if}
    {#each data.lessons.data as lesson (lesson.id)}
      <div transition:fly={{ y: 20, duration: 300 }}>
        <LessonCard {lesson} />
      </div>
    {/each}
  </div>
</section>

<section class="space-y-4">
  <H3>Decks</H3>
  <div class="grid grid-cols-1 gap-4 md:grid-cols-3">
    {#if data.decks.data.length < 1}
      <EmptySpace>No Decks</EmptySpace>
    {/if}
    {#each data.decks.data as deck (deck.id)}
      <div transition:fly={{ y: 20, duration: 300 }}>
        <DeckCard {deck} />
      </div>
    {/each}
  </div>
</section>

<svelte:head>
  <title>Dashboard | Ogonek</title>
</svelte:head>
