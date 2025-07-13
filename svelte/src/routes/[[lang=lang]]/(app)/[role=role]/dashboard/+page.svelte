<script lang="ts">
  import type { PageData } from "./$types";
  import {
    H1,
    DueTasksWidget,
    ActivityFeedWidget,
    LearnWidget,
    HeaderEmbellish,
  } from "$lib/components";
  import { getGreeting } from "$lib/utils";
  import { user } from "$lib/stores";
  import { m } from "$lib/paraglide/messages";

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

<div class="grid gap-8 lg:grid-cols-3">
  <DueTasksWidget tasks={data.tasks.data} />

  <LearnWidget
    cardsCount={data.learn.dueCards}
    streakDays={data.learn.stats.currentSterak}
    todayStudied={data.learn.stats.cardsStudiedToday}
  />
  <ActivityFeedWidget activities={data.activity} />
</div>
<svelte:head>
  <title>Dashboard | Ogonek</title>
</svelte:head>
