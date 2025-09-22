<script lang="ts">
  import { page } from "$app/state";
  import type { EventSmall } from "$lib/types/api/calendar";
  import { formatEventTime, getVideoCallService } from "$lib/utils";
  import { Caption1, Title3 } from "../typography";
  import Headline from "../typography/Headline.svelte";

  const { event }: { event: EventSmall } = $props();

  const videoCallService = event.location
    ? getVideoCallService(event.location)
    : null;
</script>

<a
  class="ring-default flex h-20 items-center justify-between gap-3 overflow-clip rounded-2xl p-2.5 shadow-sm hover:bg-stone-100/80 dark:hover:bg-stone-900"
  href="{page.params.day}/{event.id}"
>
  <Title3>
    {formatEventTime(event.dtstartTime, event.dtendTime ?? "")}
  </Title3>

  <div class="items-end text-right">
    <Headline styling={event.status === "cancelled" ? "line-through" : ""}>
      {page.params.role === "t" ? event.title : "Занятие"}
    </Headline>

    {#if event.location}
      {#if videoCallService}
        <Caption1>{videoCallService}</Caption1>
      {:else}
        <Caption1>
          {event.location}
        </Caption1>
      {/if}
    {/if}
  </div>
</a>
