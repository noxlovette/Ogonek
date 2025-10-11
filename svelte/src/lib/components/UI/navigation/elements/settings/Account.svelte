<script lang="ts">
  import { CircleUserRound } from "@lucide/svelte";
  import SidebarItem from "../SidebarItem.svelte";
  import { page } from "$app/state";
</script>

<SidebarItem
  href={`/${page.params.role}/settings/account`}
  name="Основные"
  Icon={CircleUserRound}
/>
