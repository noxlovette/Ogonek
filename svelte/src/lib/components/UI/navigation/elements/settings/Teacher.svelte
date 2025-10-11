<script lang="ts">
  import { Coffee } from "@lucide/svelte";
  import SidebarItem from "../SidebarItem.svelte";

  import MobileMenuElement from "../../mobileMenu/MobileMenuElement.svelte";
  import { page } from "$app/state";

  const href = `/${page.params.role}/settings/teacher`;
  const name = "Преподаватель";
</script>

<SidebarItem {href} {name} Icon={Coffee} />

<MobileMenuElement {href} {name} Icon={Coffee} />
