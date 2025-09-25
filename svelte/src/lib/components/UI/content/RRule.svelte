<script lang="ts">
  import { Caption1 } from "$lib/components/typography";
  import { parseRRuleDays, WEEKDAYS } from "$lib/utils";

  let { rrule, compact = false }: { rrule: string | null; compact?: boolean } =
    $props();

  // Une seule source de vérité
  const parsedData = $derived.by(() => {
    if (!rrule) return { indices: [], days: [], readable: "" };

    const indices = parseRRuleDays(rrule);
    const days = WEEKDAYS.filter((day) => indices.includes(day.index)).map(
      (day) => (compact ? day.label : day.full),
    );

    let readable = "";
    if (indices.length === 0) {
      readable = "";
    } else if (indices.length === 7) {
      readable = "Всю неделю";
    } else if (
      indices.length === 5 &&
      indices.every((day) => day >= 1 && day <= 5)
    ) {
      readable = "Рабочие дни";
    } else if (
      indices.length === 2 &&
      indices.includes(0) &&
      indices.includes(6)
    ) {
      readable = "Выходные";
    } else if (days.length === 1) {
      readable = `Каждый ${days[0].toLowerCase()}`;
    } else if (days.length === 2) {
      readable = `${days[0]} и ${days[1]}`;
    } else {
      readable = `${days.slice(0, -1).join(", ")} и ${days[days.length - 1]}`;
    }

    return { indices, days, readable };
  });
</script>

{#if rrule && parsedData.readable}
  <div class="flex items-center gap-2">
    {#if compact}
      <div class="flex gap-1">
        {#each parsedData.days as day}
          <span
            class="bg-accent/10 text-accent rounded-full px-2 py-0.5 text-xs font-medium"
          >
            {day}
          </span>
        {/each}
      </div>
    {:else}
      <Caption1>{parsedData.readable}</Caption1>
    {/if}
  </div>
{/if}
