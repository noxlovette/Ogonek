<script lang="ts">
  import type { EventSmall } from "$lib/types/api/calendar";
  import EventCard from "../cards/EventCard.svelte";
  import { EmptySpace, Title2, Title3 } from "../typography";
  import { HStack } from "../UI";

  const { events }: { events: EventSmall[] } = $props();
</script>

<HStack>
  <Title2>События Сегодня</Title2>
  {#each events as event}
    <EventCard deactivate={true} {event} />
  {:else}
    <EmptySpace><Title3>На сегодня ничего нет</Title3></EmptySpace>
  {/each}
</HStack>
