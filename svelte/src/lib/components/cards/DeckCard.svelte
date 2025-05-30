<script lang="ts">
  import type { Deck, DeckSmall } from "$lib/types";

  import { user } from "$lib/stores";
  import CardClickable from "./CardClickable.svelte";

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
</script>

<CardClickable {href} {title} {badgeText} badgeIterate={true}></CardClickable>
