<script lang="ts">
  import {
    LargeTitle,
    DueTasksWidget,
    LearnWidget,
    Toolbar,
    Divider,
    Merger,
    UniButton,
  } from "$lib/components";
  import { getGreeting } from "$lib/utils";
  import { Settings } from "@lucide/svelte";
  import CalendarFeed from "$lib/components/widgets/CalendarFeed.svelte";

  const greetingType = getGreeting();

  const greetings = {
    morning: "Доброго утра",
    afternoon: "Доброго дня",
    evening: "Вечер добрый",
    night: "Чего не спим",
  };

  let { data } = $props();
</script>

<Toolbar>
  <div class="md:hidden">
    <LargeTitle>Главная</LargeTitle>
  </div>
  <div class="hidden md:block">
    <LargeTitle>{greetings[greetingType]}, {data.user.name}</LargeTitle>
  </div>
  <Divider />
  <Merger>
    <UniButton
      fill={false}
      content="Настройки"
      href="settings/account"
      Icon={Settings}
    ></UniButton>
  </Merger>
</Toolbar>

<div class="grid items-stretch gap-8 lg:grid-cols-3">
  <DueTasksWidget tasks={data.tasks} />
  <LearnWidget cardsCount={data.badges.dueCards || 0} />
  <CalendarFeed events={data.events} />
</div>
<svelte:head>
  <title>Главная | Ogonek</title>
</svelte:head>
