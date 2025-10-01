<script lang="ts">
  import { goto } from "$app/navigation";
  import { page } from "$app/state";
  import {
    Caption1,
    EmptySpace,
    EventCard,
    HStack,
    NewCard,
    Title1,
  } from "$lib/components";
  import { formatDate } from "$lib/utils";
  import { Squirrel } from "lucide-svelte";
  import type { PageProps } from "./$types";
  let { data }: PageProps = $props();

  const sortedEvents = $derived(
    data.dayEvents.sort(
      (a, b) =>
        new Date(a.dtstartTime).getTime() - new Date(b.dtstartTime).getTime(),
    ),
  );
</script>

<HStack>
  <Title1>
    {formatDate(data.date)}
  </Title1>
  <div class="flex flex-col gap-2">
    {#each sortedEvents as event}
      <EventCard {event} />
    {:else}
      <EmptySpace>
        <Squirrel></Squirrel>
        <Caption1>Нет событий</Caption1>
      </EmptySpace>
    {/each}
    <NewCard
      addCard={async () => {
        await goto(`${page.params.day}/new`);
      }}
    />
  </div>
</HStack>
