<script lang="ts">
  import { LargeTitle, Toolbar } from "$lib/components";
  import CalendarGrid from "$lib/components/UI/interactive/CalendarGrid.svelte";
  import { fade } from "svelte/transition";
  import type { LayoutProps } from "../../$types";

  let { data, children }: LayoutProps = $props();
</script>

<CalendarGrid events={data.events} />
<div class="relative flex">
  {@render children()}
</div>
