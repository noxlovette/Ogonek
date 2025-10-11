<script lang="ts">
  import { ArrowLeft, House } from "@lucide/svelte";
  import SidebarItem from "./SidebarItem.svelte";
  import { user } from "$lib/stores";
  import MobileMenuElement from "../mobileMenu/MobileMenuElement.svelte";
  import type { UserRole } from "$lib/types";
  import { page } from "$app/state";

  function getHref(role: UserRole) {
    switch (role) {
      case "student":
        return "/s/dashboard";
      case "teacher":
        return "/t/dashboard";
      default:
        return "/admin/dashboard";
    }
  }
  let href = getHref($user.role);
  const isSettings = $derived(page.url.pathname.includes("settings"));
</script>

<SidebarItem {href} name="Главная" Icon={isSettings ? ArrowLeft : House} />
<MobileMenuElement {href} name="Главная" Icon={House} />
