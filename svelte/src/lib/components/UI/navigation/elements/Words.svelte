<script lang="ts">
  import { GraduationCap, WholeWord } from "lucide-svelte";
  import SidebarItem from "./SidebarItem.svelte";
  import { user } from "$lib/stores";
  import { getContext } from "svelte";
  import { m } from "$lib/paraglide/messages";
  import MobileMenuElement from "../mobileMenu/MobileMenuElement.svelte";

  let href = $user.role === "teacher" ? "/t/flashcards" : "/s/flashcards";

  const deckCount = getContext<number>("deckCount");
</script>

<SidebarItem
  {href}
  Icon={GraduationCap}
  name={m.flashcards()}
  badge={deckCount}
/>
<MobileMenuElement
  {href}
  Icon={GraduationCap}
  name={m.flashcards()}
  badge={deckCount}
/>
