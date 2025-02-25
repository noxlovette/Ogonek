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
    class="dark:focus:ring-milk-700 dark:focus:border-milk-800 dark:border-milk-800 disabled:text-milk-500 border-milk-200 dark:bg-milk-950 focus:ring-cacao-500 w-full rounded-lg border px-4 py-2
          transition duration-200 focus:ring focus:outline-none
                 dark:focus:ring dark:focus:outline-none"
  >
    <option value="">Select an assignee</option>
    {#each $studentStore as student}
      <option
        value={JSON.stringify({
          assignee: student.id,
          telegramId: student.telegramId,
        })}
        selected={student.id === item.assignee}
      >
        {student.name}
      </option>
    {/each}
  </select>
</div>
