<script lang="ts">
  import { goto } from "$app/navigation";
  import { page } from "$app/state";
  import { EventCard, HStack, NewCard, Title1 } from "$lib/components";
  import { formatDate } from "$lib/utils";
  import type { PageProps } from "./$types";

  let { data }: PageProps = $props();
</script>

<HStack>
  <Title1>
    {formatDate(data.date)}
  </Title1>
  <div class="grid grid-cols-2 items-start gap-2">
    {#each data.dayEvents as event}
      <EventCard {event} />
    {/each}
    <NewCard
      addCard={() => {
        goto(`${page.params.day}/new`);
      }}
    />
  </div>
</HStack>
