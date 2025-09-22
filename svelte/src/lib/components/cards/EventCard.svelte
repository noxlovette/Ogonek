<script lang="ts">
  import { page } from "$app/state";
  import type { CalendarEvent } from "$lib/types/api/calendar";
  import { formatEventTime, isVideoCallUrl } from "$lib/utils";
  import { Video } from "lucide-svelte";
  import { Caption1 } from "../typography";
  import Headline from "../typography/Headline.svelte";

  const { event }: { event: CalendarEvent } = $props();
</script>

<a
  class="ring-default flex h-30 flex-col items-start justify-between gap-3 overflow-clip rounded-2xl p-2.5 shadow-sm hover:bg-stone-100/80 dark:hover:bg-stone-900"
  href="{page.params.day}/{event.id}"
>
  <Caption1>
    {formatEventTime(event.dtstartTime, event.dtendTime ?? "", event.allDay)}
  </Caption1>

  <div>
    <Headline styling={event.status === "cancelled" ? "line-through" : ""}>
      {page.params.role === "t" ? event.title : event.organiserName}
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
