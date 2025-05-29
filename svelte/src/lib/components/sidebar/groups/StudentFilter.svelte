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
      class="focus:border-cacao-500 focus:ring-cacao-500/20 dark:focus:border-cacao-500 dark:focus:ring-cacao-500/30 w-full appearance-none rounded-lg bg-white py-2 pr-10 pl-4 text-sm shadow-sm focus:ring-2 focus:outline-none dark:border-stone-600/30 dark:bg-stone-950"
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
