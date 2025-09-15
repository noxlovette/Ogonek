<script lang="ts">
  import { Calendar, FileText } from "lucide-svelte";
  import SidebarItem from "./SidebarItem.svelte";
  import { page } from "$app/state";
</script>

<SidebarItem
  href="/{page.params.role}/calendar"
  name="Календарь"
  Icon={Calendar}
/>
