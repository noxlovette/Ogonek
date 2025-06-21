<script lang="ts">
  const { urgency = "normal", badgeText = "badge", styling = "" } = $props();

  function getBadgeClass(urgency: string): string {
    return (
      {
        overdue: "bg-red-500/20 text-red-800 dark:text-red-300 ring-red-400",
        urgent:
          "bg-orange-500/20 text-orange-800 dark:text-orange-300 ring-orange-400",
        soon: "bg-yellow-500/20 text-yellow-800 dark:text-yellow-300 ring-yellow-400",
        normal:
          "bg-sky-500/20 text-sky-700 dark:text-sky-400 dark:bg-sky-700/20 dark:ring-sky-600 ring-sky-400",
      }[urgency] ?? ""
    );
  }
  const badgeClass = getBadgeClass(urgency);
</script>

<div class="top-3 right-3 z-10 flex">
  <span
    class={`
      inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-medium
      ring-1 backdrop-blur-sm ring-inset
      ${badgeClass} ${styling}
    `}
  >
    {urgency === "overdue"
      ? "⚠️ OVERDUE"
      : urgency === "urgent"
        ? "🔥 DUE TODAY"
        : urgency === "soon"
          ? "⏰ DUE SOON"
          : badgeText}
  </span>
</div>
