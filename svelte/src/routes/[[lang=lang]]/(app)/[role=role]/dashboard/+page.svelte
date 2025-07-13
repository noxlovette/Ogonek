<script lang="ts">
  import type { PageData } from "./$types";
  import { H1, TaskCard, EmptySpace, UniButton } from "$lib/components";
  import { getGreeting } from "$lib/utils";
  import { user } from "$lib/stores";
  import HeaderEmbellish from "$lib/components/typography/HeaderEmbellish.svelte";
  import { m } from "$lib/paraglide/messages";
  import ActivityFeed from "$lib/components/cards/ActivityFeed.svelte";
  import RequestHw from "$lib/components/UI/interactive/RequestHW.svelte";
  import Learn from "$lib/components/UI/interactive/Learn.svelte";

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
  <ActivityFeed activities={data.activity} />

  <RequestHw tasks={data.tasks.data} />

  <Learn
    cardsCount={data.learn.dueCards}
    streakDays={data.learn.stats.currentSterak}
    todayStudied={data.learn.stats.cardsStudiedToday}
  />
</div>
<svelte:head>
  <title>Dashboard | Ogonek</title>
</svelte:head>
