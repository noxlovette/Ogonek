<script lang="ts">
  import { page } from "$app/state";
  import type { CalendarEvent } from "$lib/types/api/calendar";
  import { isVideoCallUrl } from "$lib/utils";
  import { Video } from "lucide-svelte";
  import { Caption1 } from "../typography";
  import Headline from "../typography/Headline.svelte";
  import { Header, UniButton } from "../UI";
  import Merger from "../UI/toolbar/Merger.svelte";

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
  class="ring-default flex flex-col items-start justify-between gap-3 overflow-clip rounded-2xl p-2.5 shadow-sm hover:bg-stone-100/80 dark:hover:bg-stone-900"
  href="{page.params.day}/{event.uid}"
>
  <Caption1>
    {formatEventTime(event.dtstart, event.dtend ?? "", event.allDay)}
  </Caption1>

  <div>
    <Headline styling={event.status === "cancelled" ? "line-through" : ""}>
      {page.params.role === "t" ? event.summary : event.organiserName}
    </Headline>

    {#if event.location}
      {#if isVideoCallUrl(event.location)}
        <Caption1>Онлайн</Caption1>
      {:else}
        <Caption1>
          {event.location}
        </Caption1>
      {/if}
    {/if}
  </div>
</a>
