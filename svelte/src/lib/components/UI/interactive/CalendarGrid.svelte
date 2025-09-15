<script lang="ts">
  import { Caption1, Title1 } from "$lib/components/typography";
  import type { CalendarEvent } from "$lib/types/api/calendar";
  import { getLocaleFromCookie } from "$lib/utils";

  const { events }: { events: CalendarEvent[] } = $props();

  let monthDays = [];

  type MonthDay = {
    day?: number;
    isCurrentMonth: boolean;
    isToday: boolean;
    events: CalendarEvent[];
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

  // Helper to get events for a specific day
  const getEventsForDay = (day: number, month: number, year: number) => {
    const dateKey = `${year}-${month}-${day}`;
    return eventsByDate[dateKey] || [];
  };

  // Empty cells before month starts
  for (let i = 0; i < startDayOfWeek; i++) {
    monthDays.push({
      day: null,
      isCurrentMonth: false,
      isToday: false,
      events: [],
    });
  }

  // Current month days with their events
  for (let day = 1; day <= lastDay.getDate(); day++) {
    const isToday =
      day === now.getDate() &&
      firstDay.getMonth() === now.getMonth() &&
      firstDay.getFullYear() === now.getFullYear();

    monthDays.push({
      day,
      isCurrentMonth: true,
      isToday,
      events: getEventsForDay(day, firstDay.getMonth(), firstDay.getFullYear()),
    });
  }

  // Fill remaining cells for next month
  const remainingCells = 42 - monthDays.length;
  const nextMonth = new Date(now.getFullYear(), now.getMonth() + 1, 1);
  for (let i = 1; i <= remainingCells; i++) {
    monthDays.push({
      day: i,
      isCurrentMonth: false,
      isToday: false,
      events: getEventsForDay(i, nextMonth.getMonth(), nextMonth.getFullYear()),
    });
  }
</script>

<Title1 styling="capitalize">
  {monthName}
  {year}
</Title1>
<div class="grid grid-cols-7 gap-1">
  {#each ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"] as dayLabel}
    <Caption1 styling="text-right">{dayLabel}</Caption1>
  {/each}
</div>

<div class="grid grid-cols-7">
  {#each monthDays as day}
    <div
      class="ring-default flex aspect-3/2 flex-col items-end p-1
             {day.isCurrentMonth
        ? ' cursor-pointer hover:bg-stone-100 '
        : 'cursor-default text-stone-400'}"
    >
      <div
        class="rounded-full p-0.5 px-1 font-medium {day.isToday
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
    </div>
  {/each}
</div>
