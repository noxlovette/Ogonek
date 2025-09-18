<script lang="ts">
  import { GraduationCap, WholeWord } from "lucide-svelte";
  import SidebarItem from "./SidebarItem.svelte";
  import { user } from "$lib/stores";
  import { getContext } from "svelte";
  import { m } from "$lib/paraglide/messages";
  import MobileMenuElement from "../mobileMenu/MobileMenuElement.svelte";
  import { writable } from "svelte/store";
  import { page } from "$app/state";

  const href = writable<string>(`/${page.params.role}/flashcards`);
  $effect(() => {
    const path = page.url.pathname;
    if (path.includes("/flashcards")) {
      $href = path;
    }
  });
  const deckCount = getContext<number>("deckCount");
</script>

<SidebarItem
  href={$href}
  Icon={GraduationCap}
  name={m.flashcards()}
  badge={deckCount}
/>
<MobileMenuElement
  href={$href}
  Icon={GraduationCap}
  name={m.flashcards()}
  badge={deckCount}
/>
