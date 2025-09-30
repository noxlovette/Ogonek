<script lang="ts">
  import { ListTodo } from "lucide-svelte";
  import SidebarItem from "./SidebarItem.svelte";
  import { getContext } from "svelte";
  import { m } from "$lib/paraglide/messages";
  import MobileMenuElement from "../mobileMenu/MobileMenuElement.svelte";
  import { page } from "$app/state";
  import { writable } from "svelte/store";
  const taskCount = getContext<number>("taskCount");

  const href = writable<string>(`/${page.params.role}/tasks`);
  $effect(() => {
    const path = page.url.pathname;
    if (path.includes("/task")) {
      $href = path;
    }
  });
</script>

<SidebarItem
  dataCy="sidebar-tasks"
  href={$href}
  Icon={ListTodo}
  name={m.tasks()}
  badge={taskCount}
/>
<MobileMenuElement
  href={$href}
  Icon={ListTodo}
  name={m.tasks()}
  badge={taskCount}
/>
