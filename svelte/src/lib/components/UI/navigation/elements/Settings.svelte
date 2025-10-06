<script lang="ts">
  import { Settings } from "@lucide/svelte";
  import SidebarItem from "./SidebarItem.svelte";
  import { user } from "$lib/stores";

  let href = $user.role === "teacher" ? "/t/settings" : "/s/settings";
</script>

<SidebarItem {href} Icon={Settings} name="Настройки" />
