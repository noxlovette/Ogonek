<script lang="ts">
  import { Book } from "lucide-svelte";
  import SidebarItem from "./SidebarItem.svelte";
</script>

<SidebarItem
  external={true}
  rightBar={true}
  href="https://dictionary.cambridge.org/"
  Icon={Book}
  name="Dictionary"
/>
