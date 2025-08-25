<script lang="ts">
  import { House } from "lucide-svelte";
  import SidebarItem from "./SidebarItem.svelte";
  import { user } from "$lib/stores";
  import { m } from "$lib/paraglide/messages";
  import MobileMenuElement from "../mobileMenu/MobileMenuElement.svelte";

  let href = $user.role === "teacher" ? "/t/dashboard" : "/s/dashboard";
</script>

<SidebarItem {href} name={m.dashboardName()} Icon={House} />
<MobileMenuElement {href} name={m.dashboardName()} Icon={House} />
