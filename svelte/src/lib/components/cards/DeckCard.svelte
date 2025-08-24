<script lang="ts">
  import type { DeckSmall } from "$lib/types";

  import { user } from "$lib/stores";
  import CardClickable from "./CardClickable.svelte";
  import Badge from "./Badge.svelte";
  import { Title3 } from "../typography";
  import { Circle, CircleCheckBig } from "lucide-svelte";
  import SeenBadge from "./SeenBadge.svelte";

  interface Props {
    deck: DeckSmall;
  }

  let { deck }: Props = $props();
  let href =
    $user.role === "teacher"
      ? `/t/flashcards/${deck.id}`
      : `/s/flashcards/${deck.id}`;

  const title = deck.title;
  const badgeText = deck.description ?? "Deck Description";
</script>

<CardClickable {href}>
  <div class="inline-flex justify-between">
    <Title3>{title}</Title3>
    {#if deck.isSubscribed}
      <CircleCheckBig />
    {:else}
      <Circle />
    {/if}
  </div>
  <div class="flex flex-wrap gap-0.5">
    {#each badgeText.split(";") as badgeCnunk, index (index)}
      <Badge badgeText={badgeCnunk}></Badge>
    {/each}
  </div>

  <SeenBadge seen={deck.seen} />
</CardClickable>
