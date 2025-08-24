<script lang="ts">
  import { Caption1 } from "../typography";

  const { urgency = "normal", styling = "", children } = $props();

  function getBadgeConfig(urgency: string) {
    switch (urgency) {
      case "overdue":
        return {
          text: "Overdue",
          color: "bg-secondary/20 text-secondary-foreground ring-secondary/40",
        };
      case "urgent":
        return {
          text: "Due Today",
          color: "bg-secondary/20 text-secondary-foreground ring-secondary/40",
        };
      case "soon":
        return {
          text: "Due Soon",
          color: "bg-secondary/20 text-secondary-foreground ring-secondary/40",
        };
      case "normal":
      default:
        return {
          color:
            "bg-secondary/10 text-secondary-foreground/80 ring-secondary/30",
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
    <Caption1>
      {#if badgeConfig.text}
        {badgeConfig.text}
      {:else}
        {@render children()}
      {/if}
    </Caption1>
  </span>
</div>
