<script lang="ts">
  import { page } from "$app/state";
  import { Caption1, LargeTitle, Title1 } from "$lib/components/typography";
  import type { EventSmall } from "$lib/types/api/calendar";
  import { getLocaleFromCookie } from "$lib/utils";
  import { generateMonthDays } from "$lib/utils/time/calendar";
  import { HStack, Toolbar } from "..";
  import Divider from "../toolbar/Divider.svelte";
  import VStack from "../VStack.svelte";

  const { events }: { events: EventSmall[] } = $props();

  const selectedDate = $derived.by(() => {
    if (page.params.year && page.params.month && page.params.day) {
      return new Date(
        Number(page.params.year),
        Number(page.params.month) - 1,
        Number(page.params.day),
      );
    }
    return null;
  });
  const calendarData = $derived.by(() => {
    const now = new Date();
    return generateMonthDays(now, events, getLocaleFromCookie());
  });

  const { monthDays, monthName, year } = $derived(calendarData);

  const isSelectedDay = (actualDate: Date): boolean => {
    return selectedDate
      ? actualDate.getTime() === selectedDate.getTime()
      : false;
  };
</script>

<Toolbar override={true}>
  <LargeTitle styling="capitalize">{monthName} {year}</LargeTitle>
  <Divider />
  <Caption1>Московское время</Caption1>
</Toolbar>
<div class="grid grid-cols-7 gap-1">
  {#each ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"] as dayLabel}
    <Caption1 styling="text-right">{dayLabel}</Caption1>
  {/each}
</div>

<div class="grid grid-cols-7 gap-1">
  {#each monthDays as day}
    <a
      href="/{page.params
        .role}/calendar/{day.actualDate.getFullYear()}/{day.actualDate.getMonth() +
        1}/{day.actualDate.getDate()}"
      class=" flex aspect-3/2 flex-col items-end rounded-sm p-1 transition-colors duration-200
             {day.isCurrentMonth
        ? isSelectedDay(day.actualDate)
          ? 'ring-default bg-stone-200/50 dark:bg-stone-800/50'
          : 'bg-clickable ring-default'
        : 'cursor-default text-stone-400'}"
    >
      <div
        class="rounded-full px-1 font-medium {day.isToday
          ? 'bg-accent text-stone-50'
          : ''}"
      >
        {day.day || ""}
      </div>

      {#if day.events.length > 0}
        <div class="flex w-full flex-col gap-0.5">
          {#each day.events.slice(0, 3) as event}
            <div
              class="bg-accent/20 text-accent truncate rounded px-1 py-0.5 text-xs"
            >
              {page.params.role === "t" ? event.title : "Занятие"}
            </div>
          {/each}

          {#if day.events.length > 3}
            <div class="text-xs text-stone-500">
              +{day.events.length - 3} more
            </div>
          {/if}
        </div>
      {/if}
    </a>
  {/each}
</div>
