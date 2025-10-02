<script lang="ts">
  import type { EventSmall } from "$lib/types/api/calendar";
  import EventCard from "../cards/EventCard.svelte";
  import { EmptySpace, Title2 } from "../typography";
  import { HStack } from "../UI";

  const { events }: { events: EventSmall[] } = $props();
</script>

<HStack>
  <Title2>События Сегодня</Title2>
  {#each events as event}
    <EventCard deactivate={true} {event} />
  {:else}
    <EmptySpace>На сегодня ничего нет</EmptySpace>
  {/each}
</HStack>
