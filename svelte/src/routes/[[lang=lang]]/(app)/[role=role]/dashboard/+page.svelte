<script lang="ts">
  import {
    H1,
    DueTasksWidget,
    ActivityFeedWidget,
    LearnWidget,
    Toolbar,
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

  let { data } = $props();
</script>

<Toolbar>
  <H1>{greetings[greetingType]}</H1>
</Toolbar>

<div class="grid gap-8 lg:grid-cols-3">
  <DueTasksWidget tasks={data.tasks} />

  <LearnWidget
    cardsCount={data.badges.dueCards}
    streakDays={data.learn.currentStreak}
    todayStudied={data.learn.cardsStudiedToday}
  />
  <ActivityFeedWidget activities={data.activity} />
</div>
<svelte:head>
  <title>Dashboard | Ogonek</title>
</svelte:head>
