<script lang="ts">
  import { BookOpenCheck } from "@lucide/svelte";
  import SidebarItem from "./SidebarItem.svelte";
  import { getContext } from "svelte";
  import MobileMenuElement from "../mobileMenu/MobileMenuElement.svelte";
  import { writable } from "svelte/store";
  import { page } from "$app/state";

  const href = writable<string>(`/${page.params.role}/lessons`);
  $effect(() => {
    const path = page.url.pathname;
    if (path.includes("/lesson")) {
      $href = path;
    }
  });

  const lessonCount = getContext<number>("lessonCount");
</script>

<SidebarItem
  href={$href}
  dataCy="sidebar-lessons"
  Icon={BookOpenCheck}
  name="Занятия"
  badge={lessonCount}
/>

<MobileMenuElement
  href={$href}
  Icon={BookOpenCheck}
  name="Занятия"
  badge={lessonCount}
/>
