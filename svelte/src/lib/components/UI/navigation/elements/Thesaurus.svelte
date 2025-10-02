<script lang="ts">
  import { BookCopy } from "@lucide/svelte";
  import SidebarItem from "./SidebarItem.svelte";
  import { m } from "$lib/paraglide/messages";
</script>

<SidebarItem
  external={true}
  href="https://www.thesaurus.com/"
  Icon={BookCopy}
  name={m.spicy_strong_lamb_tear()}
/>
