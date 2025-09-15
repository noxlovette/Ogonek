<script lang="ts">
  import { page } from "$app/state";
  import { EventCard, Title1 } from "$lib/components";
  import type { PageProps } from "./$types";

  let { data }: PageProps = $props();
</script>

<div class="flex flex-col">
  <Title1>
    {new Date(page.params.iso || "")}
  </Title1>
  <div class="flex flex-col">
    {#each data.dayEvents as event}
      <EventCard {event} />
    {/each}
  </div>
</div>
