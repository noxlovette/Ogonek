<script lang="ts">
  import type { Deck, DeckSmall } from "$lib/types";

  import { user } from "$lib/stores";
  import CardClickable from "./CardClickable.svelte";
  import Badge from "./Badge.svelte";
  import { H3 } from "../typography";
  import { Circle, CircleCheckBig } from "lucide-svelte";

  interface Props {
    deck: Deck | DeckSmall;
  }

  let { deck }: Props = $props();
  let href =
    $user.role === "teacher"
      ? `/t/words/w/${deck.id}`
      : `/s/words/w/${deck.id}`;

  const title = deck.name;
  const badgeText = deck.description ?? "Deck Description";

  console.log(deck);
</script>

<CardClickable {href}>
  <div class="inline-flex justify-between">
    <H3>{title}</H3>
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
</CardClickable>
