<script lang="ts">
  import CalendarGrid from "$lib/components/UI/interactive/CalendarGrid.svelte";
  import type { LayoutProps } from "../../$types";

  let { data, children }: LayoutProps = $props();
</script>

<CalendarGrid events={data.events} />
<div class="relative flex">
  {@render children()}
</div>
