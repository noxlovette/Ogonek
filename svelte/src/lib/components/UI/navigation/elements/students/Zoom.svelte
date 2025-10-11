<script lang="ts">
  import { Video } from "@lucide/svelte";
  import SidebarItem from "../SidebarItem.svelte";

  const { href } = $props();
</script>

<SidebarItem {href} Icon={Video} name="Zoom" external={true} />
