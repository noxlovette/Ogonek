<script lang="ts">
  import H3 from "../typography/H3.svelte";
  import type { UrgencyLevel } from "$lib/types";
  import Badge from "./Badge.svelte";

  let {
    href,
    title = "Title",
    badgeText = "Badge",
    badgeIterate = false,
    height = "150px",
    styling = "",
    prefetch = true,
    ariaLabel = "",
    urgency = "normal" as UrgencyLevel,
  }: {
    href: string;
    title?: string;
    badgeText?: string;
    badgeIterate?: boolean;
    height?: string;
    styling?: string;
    prefetch?: boolean;
    ariaLabel?: string;
    urgency?: UrgencyLevel;
  } = $props();
</script>

<a
  {href}
  aria-label={ariaLabel}
  data-sveltekit-prefetch={prefetch ? "" : null}
  class={[
    styling,
    `group ring-default bg-default relative flex
     flex-col justify-between overflow-hidden 
     rounded-xl px-4 py-5 shadow-sm backdrop-blur-md transition-all duration-300 ease-out
     hover:scale-[1.02] hover:shadow-lg hover:backdrop-blur-lg 
  `,
  ].join(" ")}
  style={`height: ${height};`}
>
  <!-- Title with glass effect -->
  <div class="relative z-10">
    <H3 styling="text-stone-900 dark:text-stone-50 drop-shadow-sm">
      {title}
    </H3>
  </div>

  {#if badgeIterate}
    <div class="inline-flex gap-1">
      {#each badgeText.split(";") as badgeCnunk, index (index)}
        <Badge badgeText={badgeCnunk} {urgency}></Badge>
      {:else}
        <Badge {badgeText} {urgency}></Badge>
      {/each}
    </div>
  {:else}
    <Badge {badgeText} {urgency}></Badge>
  {/if}

  <!-- Animated glass shimmer effect -->
  <div
    class="
    absolute inset-0 -top-2 -bottom-2 bg-gradient-to-br from-transparent
    via-stone-50/10 to-transparent opacity-0 transition-opacity duration-300
     group-hover:opacity-100
  "
  ></div>
</a>
