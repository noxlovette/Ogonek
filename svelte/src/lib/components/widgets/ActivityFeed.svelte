<script lang="ts">
  import { m } from "$lib/paraglide/messages";
  import type { ActivityLog } from "$lib/types";
  import { EmptySpace, H2 } from "../typography";
  import ActivityCard from "../cards/ActivityCard.svelte";

  const {
    activities = [],
  }: {
    activities: ActivityLog[];
  } = $props();
</script>

<div class="flex flex-col gap-4">
  <H2>{m.tired_even_polecat_slurp()}</H2>
  {#if activities.length > 0}
    <div class="grid grid-cols-1 gap-3">
      {#each activities as activity, index (index)}
        <ActivityCard {activity} />
      {/each}
    </div>
  {:else}
    <EmptySpace>{m.empty()}</EmptySpace>
  {/if}
</div>
