<script lang="ts">
  import { Book } from "lucide-svelte";
  import SidebarItem from "./SidebarItem.svelte";
  import { m } from "$lib/paraglide/messages";
</script>

<SidebarItem
  external={true}
  href="https://dictionary.cambridge.org/"
  Icon={Book}
  name={m.dictionary()}
/>
