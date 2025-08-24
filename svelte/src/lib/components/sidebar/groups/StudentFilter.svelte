<script lang="ts">
  import { assigneeStore } from "$lib/stores";
  import type { Student } from "$lib/types";

  import { getContext } from "svelte";
  import Group from "./Group.svelte";
  import H2 from "$lib/components/typography/H2.svelte";
  import { ChevronsUpDown } from "lucide-svelte";

  const students: Student[] = getContext("students");
</script>

<Group>
  <H2>Filter</H2>
  <div class="relative mt-4 min-w-40">
    <select
      id="assignee"
      name="assignee"
      bind:value={$assigneeStore}
      class="focus:border-cacao-200 focus:ring-cacao-500/70 h-full w-full rounded-2xl border border-stone-100/60 bg-white px-4 py-2 text-base text-stone-900 placeholder-stone-400 shadow-sm transition-all focus:shadow-md focus:ring-2 focus:outline-none disabled:cursor-not-allowed disabled:opacity-60 dark:border-stone-800/60 dark:bg-stone-950 dark:text-stone-100"
    >
      <option value="">All Students</option>
      {#each students as student (student.id)}
        <option value={student.id}>{student.name}</option>
      {/each}
    </select>
    <div
      class="pointer-events-none absolute inset-y-0 right-0 flex items-center px-2 text-stone-500"
    >
      <ChevronsUpDown class="size-4"></ChevronsUpDown>
    </div>
  </div>
</Group>
