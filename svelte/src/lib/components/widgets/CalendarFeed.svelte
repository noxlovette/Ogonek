<script lang="ts">
  import type { EventSmall } from "$lib/types/api/calendar";
  import EventCard from "../cards/EventCard.svelte";
  import { EmptySpace, Title2 } from "../typography";
  import { HStack } from "../UI";

  const { events }: { events: EventSmall[] } = $props();
</script>

<HStack>
  <Title2>На сегодня</Title2>
  {#each events as event}
    <EventCard deactivate={true} {event} />
  {:else}
    <EmptySpace>Сегодня можно гулять</EmptySpace>
  {/each}
</HStack>
