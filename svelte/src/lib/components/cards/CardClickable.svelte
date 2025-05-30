<script lang="ts">
  import H3 from "../typography/H3.svelte";
  import type { UrgencyLevel } from "$lib/types";

  let {
    href,
    title = "Title",
    subtitle = "Subtitle",
    height = "150px",
    styling = "",
    prefetch = true,
    ariaLabel = "",
    urgency = "normal" as UrgencyLevel,
  }: {
    href: string;
    title?: string;
    subtitle?: string;
    height?: string;
    styling?: string;
    prefetch?: boolean;
    ariaLabel?: string;
    urgency?: UrgencyLevel;
  } = $props();
  function getBadgeClass(urgency: string): string {
    return (
      {
        overdue: "bg-red-500/20 text-red-800 dark:text-red-300 ring-red-400",
        urgent:
          "bg-orange-500/20 text-orange-800 dark:text-orange-300 ring-orange-400",
        soon: "bg-yellow-500/20 text-yellow-800 dark:text-yellow-300 ring-yellow-400",
        normal: "bg-sky-500/20 text-sky-700 dark:text-sky-300 ring-sky-400",
      }[urgency] ?? ""
    );
  }
  const badgeClass = getBadgeClass(urgency);
</script>

<a
  {href}
  aria-label={ariaLabel}
  data-sveltekit-prefetch={prefetch ? "" : null}
  class={[
    styling,
    `group ring-default bg-default relative flex
     flex-col justify-between overflow-hidden 
     rounded-xl px-4 py-5 shadow-md backdrop-blur-md transition-all duration-300 ease-out
     hover:scale-[1.02] hover:shadow-lg hover:backdrop-blur-lg 
  `,
  ].join(" ")}
  style={`height: ${height};`}
>
  <!-- Title with glass effect -->
  <div class="relative z-10">
    <H3 styling="text-stone-900 dark:text-white drop-shadow-sm">
      {title}
    </H3>
  </div>

  <!-- Due Date Badge -->
  <div class="top-3 right-3 z-10 flex">
    <span
      class={`
      inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-medium
      ring-1 backdrop-blur-sm ring-inset
      ${badgeClass}
    `}
    >
      {urgency === "overdue"
        ? "⚠️ OVERDUE"
        : urgency === "urgent"
          ? "🔥 DUE TODAY"
          : urgency === "soon"
            ? "⏰ DUE SOON"
            : subtitle}
    </span>
  </div>

  <!-- Animated glass shimmer effect -->
  <div
    class="
    absolute inset-0 -top-2 -bottom-2 bg-gradient-to-br from-transparent
    via-stone-50/10 to-transparent opacity-0 transition-opacity duration-300
     group-hover:opacity-100
  "
  ></div>
</a>
