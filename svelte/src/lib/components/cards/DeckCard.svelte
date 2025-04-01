<script lang="ts">
  import type { Deck, DeckSmall } from "$lib/types";

  import { user } from "$lib/stores";
  import CardClickable from "./CardClickable.svelte";
  import { H2 } from "../typography";

  interface Props {
    deck: Deck | DeckSmall;
  }

  let { deck }: Props = $props();

  let href =
    $user.role === "teacher"
      ? `/t/words/w/${deck.id}`
      : `/s/words/w/${deck.id}`;
</script>

<CardClickable {href}>
  <H2>
    {deck.name}
  </H2>

  <div class="flex flex-col">
    <h2
      class="text-lg font-semibold
              "
    >
      {deck.description ?? "Deck Description"}
    </h2>
  </div>
</CardClickable>
