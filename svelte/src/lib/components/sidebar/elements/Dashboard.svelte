<script lang="ts">
  import { House } from "lucide-svelte";
  import SidebarItem from "./SidebarItem.svelte";
  import { user } from "$lib/stores";

  let href = $user.role === "teacher" ? "/t/dashboard" : "/s/dashboard";
</script>

<SidebarItem {href} name="Home" Icon={House} />
