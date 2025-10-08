<script lang="ts">
  import { page } from "$app/state";
  import { Merger, UniButton } from "$lib/components";
  import { X } from "@lucide/svelte";
  import { fade } from "svelte/transition";

  const { children } = $props();
</script>

<div
  data-panel
  transition:fade={{ duration: 200 }}
  class="ring-default fixed right-0 bottom-0 z-50 h-max min-h-1/2 w-[500px] rounded-tl-2xl bg-white pb-8 shadow-2xl transition-transform duration-300 ease-out dark:bg-stone-950"
>
  <!-- Close button -->
  <Merger styling="absolute right-4 top-4 z-50">
    <UniButton
      content="Закрыть"
      href="/{page.params.role}/calendar/{page.params.year}/{page.params.month}"
      Icon={X}
    />
  </Merger>

  <!-- Content -->
  <div
    class="scrollbar-thin scrollbar-track-stone-100 scrollbar-thumb-stone-300 dark:scrollbar-track-stone-800 dark:scrollbar-thumb-stone-600 overflow-y-auto p-3.5"
  >
    {@render children()}
  </div>
</div>
