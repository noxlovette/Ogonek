<script lang="ts">
  import {
    LargeTitle,
    DueTasksWidget,
    ActivityFeedWidget,
    LearnWidget,
    Toolbar,
    Divider,
    Merger,
    UniButton,
  } from "$lib/components";
  import { getGreeting } from "$lib/utils";
  import { isLoading, user } from "$lib/stores";
  import { m } from "$lib/paraglide/messages";
  import { Settings } from "lucide-svelte";

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
  <div class="md:hidden">
    <LargeTitle>Dashboard</LargeTitle>
  </div>
  <div class="hidden md:block">
    <LargeTitle>{greetings[greetingType]}</LargeTitle>
  </div>
  <Divider />
  <Merger>
    <UniButton href="settings" Icon={Settings}>
      {m.settings()}
    </UniButton>
  </Merger>
</Toolbar>

<div class="grid items-stretch gap-8 lg:grid-cols-3">
  <DueTasksWidget tasks={data.tasks} />
  <LearnWidget cardsCount={data.badges.dueCards} />
  <ActivityFeedWidget activities={data.activity} />
</div>
<svelte:head>
  <title>Dashboard | Ogonek</title>
</svelte:head>
