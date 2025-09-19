<script lang="ts">
  import Label from "$lib/components/typography/Label.svelte";
  import { parseRRuleDays, WEEKDAYS } from "$lib/utils";
  import VStack from "../VStack.svelte";

  let { rrule }: { rrule?: string | null } = $props();

  function toggleDay(dayIndex: number) {
    console.log("toggle day");
    selectedDays = selectedDays.includes(dayIndex)
      ? selectedDays.filter((d: number) => d !== dayIndex)
      : [...selectedDays, dayIndex].sort();
  }

  function isSelected(dayIndex: number) {
    return selectedDays.includes(dayIndex);
  }

  let selectedDays = $state(parseRRuleDays(rrule));

  const rule = $derived.by(() => {
    if (selectedDays.length === 0) return "";

    const byDay = selectedDays
      .map((index: number) => WEEKDAYS.find((d) => d.index === index)?.ical)
      .filter(Boolean)
      .join(",");

    return `FREQ=WEEKLY;BYDAY=${byDay}`;
  });
</script>

<div class="flex flex-col gap-1">
  <Label>Повторения</Label>
  <VStack>
    {#each WEEKDAYS as day}
      <button
        type="button"
        class="flex h-12 w-12 items-center justify-center rounded-full font-bold {isSelected(
          day.index,
        )
          ? 'border-accent bg-accent/10 border'
          : 'bg-default ring-default hover:bg-stone-100'}"
        title={day.full}
        onclick={() => toggleDay(day.index)}
        aria-pressed={isSelected(day.index)}
        aria-label="Toggle {day.full}"
      >
        {day.label}
      </button>
    {/each}
  </VStack>
  <input type="hidden" name="rrule" value={rule} />
</div>
