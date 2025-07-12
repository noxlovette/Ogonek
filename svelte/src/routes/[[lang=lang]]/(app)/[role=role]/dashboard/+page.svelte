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

<svelte:head>
  <title>Dashboard | Ogonek</title>
</svelte:head>
