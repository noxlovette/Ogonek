<script lang="ts">
  import { page } from "$app/state";
  import type { CalendarEvent } from "$lib/types/api/calendar";
  import { Caption1 } from "../typography";
  import Headline from "../typography/Headline.svelte";
  import { Header } from "../UI";

  const { event }: { event: CalendarEvent } = $props();

  // You'll need this helper for datetime formatting
  function formatEventTime(
    dtstart: string,
    dtend?: string,
    isAllDay?: boolean,
  ): string {
    if (isAllDay) return "All day";

    const start = new Date(dtstart);
    const end = dtend ? new Date(dtend) : null;

    const timeFormat = new Intl.DateTimeFormat("en-US", {
      hour: "numeric",
      minute: "2-digit",
      hour12: true,
    });

    if (!end) return timeFormat.format(start);

    // Same day event
    if (start.toDateString() === end.toDateString()) {
      return `${timeFormat.format(start)} - ${timeFormat.format(end)}`;
    }

    // Multi-day event
    return `${timeFormat.format(start)} - ${end.toLocaleDateString()}`;
  }
</script>

<a
  class="ring-default flex flex-col items-start justify-between gap-3 rounded-2xl p-3 shadow-sm transition-shadow hover:bg-stone-200 hover:shadow-lg dark:hover:bg-stone-900"
  href="{page.params.day}/{event.uid}"
>
  <!-- Time indicator -->
  <div class="flex w-full justify-between">
    <Caption1>
      {formatEventTime(event.dtstart, event.dtend ?? "", event.allDay)}
    </Caption1>
    {#if event.status === "cancelled"}
      <Caption1 styling="text-red-500">Cancelled</Caption1>
    {/if}
  </div>

  <!-- Event content -->
  <div class="flex min-w-0 flex-1 flex-col gap-1">
    <Headline>
      {event.summary}
    </Headline>

    {#if event.description}
      <p class="line-clamp-2 text-xs text-stone-600 dark:text-stone-400">
        {event.description}
      </p>
    {/if}

    {#if event.location}
      <p class="truncate text-xs text-stone-500 dark:text-stone-500">
        📍 {event.location}
      </p>
    {/if}
  </div>
</a>
