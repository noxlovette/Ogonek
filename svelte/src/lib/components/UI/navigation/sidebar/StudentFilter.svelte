<script lang="ts">
  import { assigneeStore } from "$lib/stores";
  import type { Student } from "$lib/types";

  import { getContext } from "svelte";

  const students: Student[] = getContext("students");
</script>

<select
  id="assignee"
  name="assignee"
  bind:value={$assigneeStore}
  class="ring-default focus:border-accent focus:ring-accent w-full rounded-2xl bg-white px-4 py-2 text-base text-stone-900 placeholder-stone-400 shadow-sm focus:shadow-md focus:ring-2 focus:outline-none disabled:cursor-not-allowed disabled:opacity-60 dark:bg-stone-950 dark:text-stone-100"
>
  <option value="">Все ученики</option>
  {#each students as student (student.id)}
    <option value={student.id}>{student.name}</option>
  {/each}
</select>
<div
  class="pointer-events-none absolute inset-y-0 right-0 flex items-center px-2 text-stone-500"
></div>
