<script lang="ts">
  import { m } from "$lib/paraglide/messages";
  import type { ActivityLog } from "$lib/types";
  import { EmptySpace, H2 } from "../typography";
  import ActivityCard from "./ActivityCard.svelte";

  const {
    activities = [],
  }: {
    activities: ActivityLog[];
  } = $props();
</script>

<div class="col-span-2 flex flex-col gap-4">
  <H2>Recent Activity</H2>
  {#if activities.length > 0}
    <div class="grid grid-cols-1 gap-4 md:grid-cols-2 xl:grid-cols-3">
      {#each activities as activity, index (index)}
        <ActivityCard {activity} />
      {/each}
    </div>
  {:else}
    <EmptySpace>
      {m.noDecks()}
    </EmptySpace>
  {/if}
</div>
