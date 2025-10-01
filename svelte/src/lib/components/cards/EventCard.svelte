<script lang="ts">
  import { page } from "$app/state";
  import type { EventSmall } from "$lib/types/api/calendar";
  import { formatEventTime, getVideoCallService } from "$lib/utils";
  import { Caption1, Title3 } from "../typography";
  import Headline from "../typography/Headline.svelte";
  import { Divider, VStack } from "../UI";
  import CardClickable from "./CardClickable.svelte";

  const {
    event,
    deactivate = false,
  }: { event: EventSmall; deactivate?: boolean } = $props();

  const videoCallService = event.location
    ? getVideoCallService(event.location)
    : null;
</script>

<CardClickable {deactivate} href="{page.params.day}/{event.id}">
  <Title3>
    {formatEventTime(event.dtstartTime, event.dtendTime ?? "")}
  </Title3>
  <VStack>
    <Headline styling={event.status === "cancelled" ? "line-through" : ""}>
      {page.params.role === "t" ? event.title : "Занятие"}
    </Headline>
    {#if event.location}
      <Divider />
      {#if videoCallService}
        <Caption1>{videoCallService}</Caption1>
      {:else}
        <Caption1>
          {event.location}
        </Caption1>
      {/if}
    {/if}
  </VStack>
</CardClickable>
