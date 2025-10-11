<script lang="ts">
  import { Coffee } from "@lucide/svelte";
  import SidebarItem from "../SidebarItem.svelte";
  import { page } from "$app/state";
</script>

<SidebarItem
  href={`/${page.params.role}/settings/teacher`}
  name="Преподаватель"
  Icon={Coffee}
/>
