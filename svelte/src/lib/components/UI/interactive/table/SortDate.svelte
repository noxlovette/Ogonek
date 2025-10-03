<script lang="ts">
  import { sortBy, sortOrder } from "$lib/stores";
  import type { SortBy } from "$lib/types";
  import {
    ArrowDownAZ,
    ArrowUpAZ,
    CalendarArrowDown,
    CalendarArrowUp,
  } from "@lucide/svelte";

  const sortFields: { value: SortBy; label: string; isTime: boolean }[] = [
    { value: "created_at", label: "Создание", isTime: true },
    { value: "updated_at", label: "Обновление", isTime: true },
    { value: "title", label: "Название", isTime: false },
    { value: "due_date", label: "Срок", isTime: true },
  ];

  const isTimeSort = $derived(
    sortFields.find((f) => f.value === $sortBy)?.isTime ?? false,
  );

  const baseClass =
    "ring-default text-center hover-default rounded-full p-2 md:p-3 shadow-sm transition-all";
</script>

<div class="gap-default flex flex-wrap items-center">
  <button
    type="button"
    onclick={() => ($sortOrder = $sortOrder === "asc" ? "desc" : "asc")}
    class="{baseClass} flex items-center"
  >
    {#if isTimeSort}
      {#if $sortOrder === "asc"}
        <CalendarArrowUp class="size-5" />
      {:else}
        <CalendarArrowDown class="size-5" />
      {/if}
    {:else if $sortOrder === "asc"}
      <ArrowUpAZ class="size-5" />
    {:else}
      <ArrowDownAZ class="size-5" />
    {/if}
  </button>

  <select name="sort" bind:value={$sortBy} class={baseClass}>
    {#each sortFields as field}
      <option value={field.value}>{field.label}</option>
    {/each}
  </select>
</div>
