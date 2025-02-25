<script lang="ts">
  import { WholeWord } from "lucide-svelte";
  import SidebarItem from "./SidebarItem.svelte";
  import { user } from "$lib/stores";

  let href = $user.role === "teacher" ? "/t/words" : "/s/words";
</script>

<SidebarItem {href} Icon={WholeWord} name="Words" />
