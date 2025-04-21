<script lang="ts">
  import { Video } from "lucide-svelte";
  import SidebarItem from "./SidebarItem.svelte";
  import { teacherData } from "$lib/stores";

  let href = $teacherData.teacherZoomUrl
    ? $teacherData.teacherZoomUrl
    : "https://zoom.us";
</script>

<SidebarItem {href} Icon={Video} name="Zoom" external={true} />
