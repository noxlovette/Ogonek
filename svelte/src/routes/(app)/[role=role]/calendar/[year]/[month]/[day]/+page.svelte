<script lang="ts">
  import { EventCard, HStack, Title1 } from "$lib/components";
  import { formatDate } from "$lib/utils";
  import type { PageProps } from "./$types";

  let { data }: PageProps = $props();
</script>

<Title1>
  {formatDate(data.date)}
</Title1>
<div class="grid grid-cols-2 gap-2">
  {#each data.dayEvents as event}
    <EventCard {event} />
  {/each}
</div>
