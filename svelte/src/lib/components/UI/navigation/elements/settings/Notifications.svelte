<script lang="ts">
  import { Bell } from "@lucide/svelte";
  import SidebarItem from "../SidebarItem.svelte";
  import { page } from "$app/state";
</script>

<SidebarItem
  href={`/${page.params.role}/settings/notifications`}
  name="Уведомления"
  Icon={Bell}
/>
