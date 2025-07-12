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
  import { m } from "$lib/paraglide/messages";
  import ActivityFeed from "$lib/components/cards/ActivityFeed.svelte";

  const greetingType = getGreeting();

  const greetings = {
    morning: m["dashboard.morningGreeting"]({ name: $user.name! }),
    afternoon: m["dashboard.afternoonGreeting"]({ name: $user.name! }),
    evening: m["dashboard.eveningGreeting"]({ name: $user.name! }),
    night: m["dashboard.nightGreeting"]({ name: $user.name! }),
  };

  let { data }: { data: PageData } = $props();
  console.log(data);
</script>

<HeaderEmbellish>
  <H1>{greetings[greetingType]}</H1>
</HeaderEmbellish>

<ActivityFeed activities={data.activity} />
<section class="space-y-4">
  <H3>{m.tasks()}</H3>
  <div class="grid grid-cols-1 gap-4 md:grid-cols-3">
    {#if data.tasks.data.length < 1}
      <EmptySpace>{m.noTasks()}</EmptySpace>
    {/if}
    {#each data.tasks.data as task (task.id)}
      <div transition:fly={{ y: 20, duration: 300 }}>
        <TaskCard {task} />
      </div>
    {/each}
  </div>
</section>

<section class="space-y-4">
  <H3>{m.lessons()}</H3>
  <div class="grid grid-cols-1 gap-4 md:grid-cols-3">
    {#if data.lessons.data.length < 1}
      <EmptySpace>{m.noLessons()}</EmptySpace>
    {/if}
    {#each data.lessons.data as lesson (lesson.id)}
      <div transition:fly={{ y: 20, duration: 300 }}>
        <LessonCard {lesson} />
      </div>
    {/each}
  </div>
</section>

<section class="space-y-4">
  <H3>{m.decks()}</H3>
  <div class="grid grid-cols-1 gap-4 md:grid-cols-3">
    {#if data.decks.data.length < 1}
      <EmptySpace>{m.noDecks()}</EmptySpace>
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
