<script lang="ts">
  import type { Snippet } from "svelte";
  import { Caption1 } from "../typography";
  import type { Urgency } from "$lib/types";
  import { m } from "$lib/paraglide/messages";

  const {
    urgency = "normal",
    styling = "",
    children,
  }: { urgency?: Urgency; styling?: string; children: Snippet } = $props();

  function getBadgeConfig(urgency: Urgency) {
    switch (urgency) {
      case "overdue":
        return {
          text: m.sweet_alive_bear_pop(),
          color: "bg-red-600/12 text-red-700 dark:text-red-500 ring-red-600  ",
        };
      case "urgent":
        return {
          text: m.caring_super_meerkat_tend(),
          color:
            "bg-amber-600/12 dark:text-amber-500 text-amber-700 ring-amber-600",
        };
      case "soon":
        return {
          text: m.blue_day_tuna_pinch(),
          color:
            "bg-yellow-600/12  text-yellow-700 dark:text-yellow-500 ring-yellow-600",
        };
      case "green":
        return {
          color:
            "bg-emerald-600/12 ring-emerald-600 text-emerald-700 dark:text-emerald-500",
        };
      case "normal":
      default:
        return {
          color:
            "bg-stone-400/12 ring-stone-600/40 dark:ring-stone-100/12 dark:bg-stone-100/12",
        };
    }
  }

  const badgeConfig = getBadgeConfig(urgency);
</script>

<div class="top-3 right-3 z-10 flex">
  <span
    class={`
      inline-flex items-center rounded-full px-2.5 py-1.5 font-medium
      ring-2 backdrop-blur-sm ring-inset
      ${badgeConfig.color} ${styling}
    `}
  >
    <Caption1 styling="">
      {#if badgeConfig.text}
        {badgeConfig.text}
      {:else}
        {@render children()}
      {/if}
    </Caption1>
  </span>
</div>
