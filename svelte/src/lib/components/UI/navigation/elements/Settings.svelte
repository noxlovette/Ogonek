<script lang="ts">
  import { Settings } from "@lucide/svelte";
  import SidebarItem from "./SidebarItem.svelte";
  import { user } from "$lib/stores";
  import { m } from "$lib/paraglide/messages";

  let href = $user.role === "teacher" ? "/t/settings" : "/s/settings";
</script>

<SidebarItem {href} Icon={Settings} name={m.settings()} />
