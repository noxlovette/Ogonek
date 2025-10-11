<script lang="ts">
  import { CircleUserRound } from "@lucide/svelte";
  import SidebarItem from "../SidebarItem.svelte";
  import { page } from "$app/state";
  import MobileMenuElement from "../../mobileMenu/MobileMenuElement.svelte";

  const href = `/${page.params.role}/settings/account`;
  const name = "Основные";
</script>

<SidebarItem {href} {name} Icon={CircleUserRound} />

<MobileMenuElement {href} {name} Icon={CircleUserRound} />
