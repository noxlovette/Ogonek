<script lang="ts">
  import { studentStore } from "$lib/stores";
  import { Label } from "$lib/components/typography";

  type Assignable = { assignee?: string };

  let { item }: { item: Assignable } = $props();
</script>

<div class="space-y-2">
  <Label>Assignee</Label>
  <select
    id="assignee"
    name="student"
    class="focus:ring-cacao-500 w-full rounded-lg border border-stone-200 px-4 py-2 transition duration-200 focus:ring focus:outline-none disabled:text-stone-500
          dark:border-stone-800 dark:bg-stone-950 dark:focus:border-stone-800 dark:focus:ring
                 dark:focus:ring-stone-700 dark:focus:outline-none"
  >
    <option value="">Select an assignee</option>
    {#each $studentStore as student}
      <option
        value={JSON.stringify({
          assignee: student.id,
          studentTelegramId: student.studentTelegramId,
        })}
        selected={student.id === item.assignee}
      >
        {student.name}
      </option>
    {/each}
  </select>
</div>
