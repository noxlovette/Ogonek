<script lang="ts">
  import type { Snippet } from "svelte";
  import { Caption1 } from "../typography";
  import type { Urgency } from "$lib/types";
  const {
    urgency = "normal",
    styling = "",
    children,
  }: { urgency?: Urgency; styling?: string; children: Snippet } = $props();

  function getBadgeConfig(urgency: Urgency) {
    switch (urgency) {
      case "overdue":
        return {
          text: "Overdue",
          color: "bg-rose-600 ring-rose-700/40  ",
        };
      case "urgent":
        return {
          text: "Due Today",
          color: "bg-orange-600  ring-orange-700/40",
        };
      case "soon":
        return {
          text: "Due Soon",
          color: "bg-yellow-600   ring-yellow-600/40",
        };
      case "green":
        return {
          color: "bg-emerald-600 ring-emerald-600/40",
        };
      case "normal":
      default:
        return {
          color: "bg-secondary-dark ring-secondary/30 dark:bg-secondary",
        };
    }
  }

  const badgeConfig = getBadgeConfig(urgency);
</script>

<div class="top-3 right-3 z-10 flex">
  <span
    class={`
      inline-flex items-center rounded-full px-2.5 py-0.5
      ring-1 backdrop-blur-sm ring-inset
      ${badgeConfig.color} ${styling}
    `}
  >
    <Caption1 styling="text-white">
      {#if badgeConfig.text}
        {badgeConfig.text}
      {:else}
        {@render children()}
      {/if}
    </Caption1>
  </span>
</div>
