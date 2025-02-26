<script lang="ts">
  import { BookOpenCheck } from "lucide-svelte";
  import SidebarItem from "./SidebarItem.svelte";
  import { user } from "$lib/stores";

  let href = $user.role === "teacher" ? "/t/lessons" : "/s/lessons";
</script>

<SidebarItem {href} Icon={BookOpenCheck} name="lessons" />
