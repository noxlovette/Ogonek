<script lang="ts">
  import Label from "$lib/components/typography/Label.svelte";
  import VStack from "../VStack.svelte";

  let { rrule }: { rrule?: string | null } = $props();

  type iCalRecDay = {
    label: string;
    full: string;
    ical: string;
    index: number;
  };

  const days: iCalRecDay[] = [
    { label: "S", full: "Sunday", ical: "SU", index: 0 },
    { label: "M", full: "Monday", ical: "MO", index: 1 },
    { label: "T", full: "Tuesday", ical: "TU", index: 2 },
    { label: "W", full: "Wednesday", ical: "WE", index: 3 },
    { label: "T", full: "Thursday", ical: "TH", index: 4 },
    { label: "F", full: "Friday", ical: "FR", index: 5 },
    { label: "S", full: "Saturday", ical: "SA", index: 6 },
  ];
  function toggleDay(dayIndex: number) {
    console.log("toggle day");
    selectedDays = selectedDays.includes(dayIndex)
      ? selectedDays.filter((d: number) => d !== dayIndex)
      : [...selectedDays, dayIndex].sort();
  }

  function isSelected(dayIndex: number) {
    return selectedDays.includes(dayIndex);
  }

  let selectedDays = $state(
    (() => {
      if (rrule) {
        const byDayMatch = rrule.match(/BYDAY=([^;]+)/);
        if (byDayMatch) {
          const ruleDays = byDayMatch[1].split(",");
          return days
            .filter((day) => ruleDays.includes(day.ical))
            .map((day) => day.index);
        }
      }
      return [];
    })(),
  );

  const rule = $derived.by(() => {
    if (selectedDays.length === 0) return "";

    const byDay = selectedDays
      .map((index: number) => days.find((d) => d.index === index)?.ical)
      .filter(Boolean)
      .join(",");

    return `FREQ=WEEKLY;BYDAY=${byDay}`;
  });
</script>

<div class="flex flex-col gap-1">
  <Label>Повторения</Label>
  <VStack>
    {#each days as day}
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
