<script lang="ts">
  import { House } from "@lucide/svelte";
  import SidebarItem from "./SidebarItem.svelte";
  import { user } from "$lib/stores";
  import MobileMenuElement from "../mobileMenu/MobileMenuElement.svelte";
  import type { UserRole } from "$lib/types";

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
</script>

<SidebarItem {href} name="Главная" Icon={House} />
<MobileMenuElement {href} name="Главная" Icon={House} />
