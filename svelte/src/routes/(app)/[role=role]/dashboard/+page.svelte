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
  import { user } from "$lib/stores";
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
    <LargeTitle>{m.dashboardName()}</LargeTitle>
  </div>
  <div class="hidden md:block">
    <LargeTitle>{greetings[greetingType]}</LargeTitle>
  </div>
  <Divider />
  <Merger>
    <UniButton
      content={m.settings()}
      href="settings"
      Icon={Settings}
    ></UniButton>
  </Merger>
</Toolbar>

<div class="grid items-stretch gap-8 lg:grid-cols-3">
  <DueTasksWidget tasks={data.tasks} />
  <LearnWidget cardsCount={data.badges.dueCards || 0} />
  <ActivityFeedWidget activities={data.activity} />
</div>
<svelte:head>
  <title>{m.dashboardName()}</title>
</svelte:head>
