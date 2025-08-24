<script lang="ts">
  import { WholeWord } from "lucide-svelte";
  import SidebarItem from "./SidebarItem.svelte";
  import { user } from "$lib/stores";
  import { getContext } from "svelte";
  import { m } from "$lib/paraglide/messages";

  let href = $user.role === "teacher" ? "/t/flashcards" : "/s/flashcards";

  const deckCount = getContext<number>("deckCount");
</script>

<SidebarItem {href} Icon={WholeWord} name={m.flashcards()} badge={deckCount} />
