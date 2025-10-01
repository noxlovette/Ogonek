<script lang="ts">
  import { page } from "$app/state";
  import { Caption1, LargeTitle } from "$lib/components/typography";
  import type { EventSmall } from "$lib/types/api/calendar";
  import { getLocaleFromCookie } from "$lib/utils";
  import { generateMonthDays } from "$lib/utils/time/calendar";
  import { Calendar1, ChevronLeft, ChevronRight } from "lucide-svelte";
  import { HStack, Toolbar } from "..";
  import UniButton from "../forms/buttons/UniButton.svelte";
  import Divider from "../toolbar/Divider.svelte";
  import Merger from "../toolbar/Merger.svelte";

  const { events }: { events: EventSmall[] } = $props();

  const selectedMonth = $derived.by(() => {
    if (page.params.year && page.params.month) {
      return new Date(Number(page.params.year), Number(page.params.month) - 1);
    }
    return new Date();
  });

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
    return generateMonthDays(selectedMonth, events, getLocaleFromCookie());
  });

  const { monthDays, monthName, year } = $derived(calendarData);

  // 🔥 Navigation logic refactorisée - gère les edge cases
  const navigationUrls = $derived.by(() => {
    const currentYear = Number(page.params.year);
    const currentMonth = Number(page.params.month);

    // Next month logic
    const nextYear = currentMonth === 12 ? currentYear + 1 : currentYear;
    const nextMonthNum = currentMonth === 12 ? 1 : currentMonth + 1;

    // Previous month logic
    const prevYear = currentMonth === 1 ? currentYear - 1 : currentYear;
    const prevMonthNum = currentMonth === 1 ? 12 : currentMonth - 1;

    return {
      next: `/${page.params.role}/calendar/${nextYear}/${nextMonthNum}`,
      prev: `/${page.params.role}/calendar/${prevYear}/${prevMonthNum}`,
    };
  });

  const isSelectedDay = (actualDate: Date): boolean => {
    return selectedDate
      ? actualDate.getTime() === selectedDate.getTime()
      : false;
  };

  // 🚀 Alternative: Today button pour reset vers le mois actuel
  const todayUrl = $derived.by(() => {
    const today = new Date();
    return `/${page.params.role}/calendar/${today.getFullYear()}/${today.getMonth() + 1}`;
  });
</script>

<Toolbar override={true}>
  <HStack>
    <LargeTitle styling="capitalize">{monthName} {year}</LargeTitle>
    <Caption1>Московское время</Caption1>
    <Caption1 styling="text-rose-400"
      >Календарь находится на стадии тестирования. Возможны ошибки</Caption1
    >
  </HStack>
  <Divider />
  <Merger>
    <UniButton
      content="Предыдущий месяц"
      href={navigationUrls.prev}
      Icon={ChevronLeft}
    />
    <UniButton
      content="Сегодня"
      variant="prominent"
      href={todayUrl}
      Icon={Calendar1}
    ></UniButton>
    <UniButton
      content="Следующий месяц"
      href={navigationUrls.next}
      Icon={ChevronRight}
    />
  </Merger>
  <Divider />
</Toolbar>

<div class="grid grid-cols-7 gap-1">
  {#each ["Lun", "Mar", "Mer", "Jeu", "Ven", "Sam", "Dim"] as dayLabel}
    <Caption1 styling="text-right">{dayLabel}</Caption1>
  {/each}
</div>

<div class="grid grid-cols-7 gap-1">
  {#each monthDays as day}
    <a
      href="/{page.params
        .role}/calendar/{day.actualDate.getFullYear()}/{day.actualDate.getMonth() +
        1}/{day.actualDate.getDate()}"
      class="flex aspect-3/2 flex-col items-end rounded-sm p-1 transition-colors duration-200
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
              +{day.events.length - 3} de plus
            </div>
          {/if}
        </div>
      {/if}
    </a>
  {/each}
</div>
