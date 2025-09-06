<script lang="ts">
  import { UserRound } from "lucide-svelte";
  import SidebarItem from "./SidebarItem.svelte";
  import { m } from "$lib/paraglide/messages";
</script>

<SidebarItem
  href="/t/students"
  Icon={UserRound}
  name={m.teary_short_shrike_talk()}
/>
