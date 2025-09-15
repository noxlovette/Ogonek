<script lang="ts">
  import { page } from "$app/state";
  import { Callout, LargeTitle, Toolbar, UniButton } from "$lib/components";
  import CalendarGrid from "$lib/components/UI/interactive/CalendarGrid.svelte";
  import { X } from "lucide-svelte";
  import type { LayoutProps } from "./$types";

  let { data, children }: LayoutProps = $props();
</script>

<Toolbar>
  <LargeTitle>Календарь</LargeTitle>
</Toolbar>
<div class="relative flex">
  <CalendarGrid events={data.events} />

  {@render children()}
</div>
