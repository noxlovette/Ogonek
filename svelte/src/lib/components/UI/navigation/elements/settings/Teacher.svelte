<script lang="ts">
  import { Coffee } from "@lucide/svelte";
  import SidebarItem from "../SidebarItem.svelte";

  import MobileMenuElement from "../../mobileMenu/MobileMenuElement.svelte";
  import { page } from "$app/state";

  const href = `/${page.params.role}/settings/teacher`;
  const name = "Преподаватель";
</script>

{#if page.params.role === "t"}
  <SidebarItem {href} {name} Icon={Coffee} />

  <MobileMenuElement {href} {name} Icon={Coffee} />
{/if}
