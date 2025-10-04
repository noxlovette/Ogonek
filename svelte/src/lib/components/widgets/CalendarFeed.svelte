<script lang="ts">
  import { m } from "$lib/paraglide/messages";
  import type { EventSmall } from "$lib/types/api/calendar";
  import EventCard from "../cards/EventCard.svelte";
  import { EmptySpace, Title2 } from "../typography";
  import { HStack } from "../UI";

  const { events }: { events: EventSmall[] } = $props();
</script>

<HStack>
  <Title2>{m.smug_bright_thrush_praise()}</Title2>
  {#each events as event}
    <EventCard deactivate={true} {event} />
  {:else}
    <EmptySpace>{m.pink_still_crab_view()}</EmptySpace>
  {/each}
</HStack>
