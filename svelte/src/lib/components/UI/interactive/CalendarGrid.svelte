<script lang="ts">
  import { page } from "$app/state";
  import { Caption1, Title1 } from "$lib/components/typography";
  import type { CalendarEvent } from "$lib/types/api/calendar";
  import { getLocaleFromCookie } from "$lib/utils";
  import { HStack } from "..";

  const { events }: { events: CalendarEvent[] } = $props();

  let monthDays: MonthDay[] = [];

  type MonthDay = {
    day: number;
    isCurrentMonth: boolean;
    isToday: boolean;
    events: CalendarEvent[];
    actualDate: Date;
  };
  const now = new Date();
  const firstDay = new Date(now.getFullYear(), now.getMonth(), 1);
  const lastDay = new Date(now.getFullYear(), now.getMonth() + 1, 0);
  const monthName = firstDay.toLocaleDateString(getLocaleFromCookie(), {
    month: "long",
  });
  const year = firstDay.getFullYear();

  // Group events by date for O(1) lookup instead of filtering on each render
  const eventsByDate = events.reduce(
    (acc, event) => {
      const eventDate = new Date(event.dtstart);
      const dateKey = `${eventDate.getFullYear()}-${eventDate.getMonth()}-${eventDate.getDate()}`;
      if (!acc[dateKey]) acc[dateKey] = [];
      acc[dateKey].push(event);
      return acc;
    },
    {} as Record<string, typeof events>,
  );

  const startDayOfWeek = firstDay.getDay();
  const prevMonth = new Date(now.getFullYear(), now.getMonth() - 1, 1);
  const prevMonthLastDay = new Date(
    now.getFullYear(),
    now.getMonth(),
    0,
  ).getDate();

  const getEventsForDay = (day: number, month: number, year: number) => {
    const dateKey = `${year}-${month}-${day}`;
    return eventsByDate[dateKey] || [];
  };

  for (let i = 0; i < startDayOfWeek; i++) {
    const dayNum = prevMonthLastDay - startDayOfWeek + i + 1;
    monthDays.push({
      day: dayNum,
      isCurrentMonth: false,
      isToday: false,
      events: [],
      actualDate: new Date(
        prevMonth.getFullYear(),
        prevMonth.getMonth(),
        dayNum,
      ),
    });
  }

  for (let day = 1; day <= lastDay.getDate(); day++) {
    const actualDate = new Date(
      firstDay.getFullYear(),
      firstDay.getMonth(),
      day,
    );
    monthDays.push({
      day,
      isCurrentMonth: true,
      isToday:
        day === now.getDate() &&
        firstDay.getMonth() === now.getMonth() &&
        firstDay.getFullYear() === now.getFullYear(),
      events: getEventsForDay(day, firstDay.getMonth(), firstDay.getFullYear()),
      actualDate,
    });
  }

  const remainingCells = 42 - monthDays.length;
  const nextMonth = new Date(now.getFullYear(), now.getMonth() + 1, 1);
  for (let i = 1; i <= remainingCells; i++) {
    const actualDate = new Date(
      nextMonth.getFullYear(),
      nextMonth.getMonth(),
      i,
    );
    monthDays.push({
      day: i,
      isCurrentMonth: false,
      isToday: false,
      events: getEventsForDay(i, nextMonth.getMonth(), nextMonth.getFullYear()),
      actualDate,
    });
  }

  const selectedDate = $derived.by(() => {
    if (page.params.year && page.params.month && page.params.day) {
      return new Date(
        Number(page.params.year),
        Number(page.params.month) - 1,
        Number(page.params.day),
      );
    }
  });

  const isSelectedDay = (actualDate: Date): boolean => {
    return actualDate.getTime() === selectedDate?.getTime();
  };
</script>

<HStack>
  <Title1 styling="capitalize">
    {monthName}
    {year}
  </Title1>
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
        class=" flex aspect-3/2 flex-col items-end rounded-sm p-1
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
                {event.summary}
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
</HStack>
