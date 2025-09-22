<script lang="ts">
  import { LargeTitle, Toolbar } from "$lib/components";
  import CalendarGrid from "$lib/components/UI/interactive/CalendarGrid.svelte";
  import type { LayoutProps } from "../../$types";

  let { data, children }: LayoutProps = $props();
</script>

<Toolbar override={true}>
  <LargeTitle>Календарь</LargeTitle>
</Toolbar>
<div class="relative flex">
  <CalendarGrid events={data.events} />

  {@render children()}
</div>
