<script lang="ts">
  import { Caption1 } from "../typography";

  const { urgency = "normal", styling = "", children } = $props();

  function getBadgeConfig(urgency: string) {
    switch (urgency) {
      case "overdue":
        return {
          text: "Overdue",
          color: "bg-red-600 ring-red-700/40  ",
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
