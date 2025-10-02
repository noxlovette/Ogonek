<script lang="ts">
  import { AudioLines } from "@lucide/svelte";
  import SidebarItem from "./SidebarItem.svelte";
  import { m } from "$lib/paraglide/messages";
</script>

<SidebarItem
  external={true}
  href="https://forvo.com/"
  Icon={AudioLines}
  name={m.suave_game_fox_emerge()}
/>
