<script lang="ts">
  import { Video } from "lucide-svelte";
  import SidebarItem from "./SidebarItem.svelte";
  import { profile } from "$lib/stores";

  let href = $profile.zoomUrl ? $profile.zoomUrl : "https://zoom.us";
</script>

<SidebarItem {href} Icon={Video} name="Zoom" external={true} />
