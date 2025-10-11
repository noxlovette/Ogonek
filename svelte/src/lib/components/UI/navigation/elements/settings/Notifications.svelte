<script lang="ts">
  import { Bell } from "@lucide/svelte";
  import SidebarItem from "../SidebarItem.svelte";
  import { page } from "$app/state";
  import MobileMenuElement from "../../mobileMenu/MobileMenuElement.svelte";

  const href = `/${page.params.role}/settings/notifications`;
  const name = "Уведомления";
</script>

<SidebarItem {href} {name} Icon={Bell} />

<MobileMenuElement {href} {name} Icon={Bell} />
