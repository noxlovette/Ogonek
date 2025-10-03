<script lang="ts">
  import { goto } from "$app/navigation";
  import { page } from "$app/state";
  import {
    EmptySpace,
    EventCard,
    HStack,
    NewCard,
    Title1,
  } from "$lib/components";
  import { formatDateOnly } from "$lib/utils";
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
    {formatDateOnly(data.date)}
  </Title1>
  <div class="flex flex-col gap-2">
    {#each sortedEvents as event}
      <EventCard {event} />
    {:else}
      <EmptySpace>Нет событий</EmptySpace>
    {/each}
    <NewCard
      addCard={async () => {
        await goto(`${page.params.day}/new`);
      }}
    />
  </div>
</HStack>
