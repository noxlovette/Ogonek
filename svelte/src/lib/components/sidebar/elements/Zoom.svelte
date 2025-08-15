<script lang="ts">
  import { Video } from "lucide-svelte";
  import SidebarItem from "./SidebarItem.svelte";
  import { getContext } from "svelte";

  let href = getContext<string>("callURL")
    ? getContext<string>("callURL")
    : "https://zoom.us";
</script>

<SidebarItem {href} Icon={Video} name="Zoom" external={true} />
