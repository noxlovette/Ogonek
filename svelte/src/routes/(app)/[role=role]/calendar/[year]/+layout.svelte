<script lang="ts">
  import { page } from "$app/state";
  import { Merger, UniButton } from "$lib/components";
  import { X, ChevronLeft, ChevronRight } from "lucide-svelte";
  import { panelSide } from "$lib/stores";
  import type { LayoutProps } from "./$types";

  let { data, children }: LayoutProps = $props();

  function toggleSide() {
    panelSide.set($panelSide === "left" ? "right" : "left");
  }
</script>

<div
  data-panel
  class="ring-default fixed inset-y-0 z-50 w-[500px] bg-white shadow-2xl transition-transform duration-300 ease-out
         dark:bg-stone-950
         {$panelSide === 'left'
    ? 'left-0 translate-x-0 rounded-r-2xl'
    : 'right-0 translate-x-0 rounded-l-2xl'}"
>
  <!-- Toggle side button -->
  <button
    onclick={toggleSide}
    class="bg-accent hover:bg-accent-600 absolute top-1/2 -translate-y-1/2 rounded-full p-2
           text-white transition-all duration-200 hover:scale-110
           {$panelSide === 'left' ? '-right-6' : '-left-6'}"
    aria-label="Toggle panel side"
  >
    {#if $panelSide === "left"}
      <ChevronRight size={16} />
    {:else}
      <ChevronLeft size={16} />
    {/if}
  </button>

  <!-- Close button -->
  <Merger styling="absolute right-4 top-4 z-50">
    <UniButton href="/{page.params.role}/calendar" Icon={X} />
  </Merger>

  <!-- Content -->
  <div
    class="scrollbar-thin scrollbar-track-stone-100 scrollbar-thumb-stone-300 dark:scrollbar-track-stone-800 dark:scrollbar-thumb-stone-600 h-full overflow-y-auto p-4 pb-8"
  >
    {@render children()}
  </div>
</div>
