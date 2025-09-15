<script lang="ts">
  import { Callout, LargeTitle, Toolbar } from "$lib/components";
  import CalendarGrid from "$lib/components/UI/interactive/CalendarGrid.svelte";
  import type { LayoutProps } from "./$types";

  let { data, children }: LayoutProps = $props();
</script>

<Toolbar>
  <LargeTitle>Календарь</LargeTitle>
</Toolbar>

<CalendarGrid events={data.events} />
{@render children()}
