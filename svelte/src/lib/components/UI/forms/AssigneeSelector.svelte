<script lang="ts">
  import { studentStore, assigneeStore, user } from "$lib/stores";
  import { Label } from "$lib/components/typography";
  type Assignable = { assignee?: string };
  let { item }: { item: Assignable } = $props();
</script>

<div class="flex flex-col space-y-1">
  <Label>Assignee</Label>
  <select
    id="assignee"
    name="student"
    class="focus:border-cacao-200 focus:ring-cacao-500/70 h-full w-full rounded-2xl border border-stone-100/60 bg-white px-4 py-2 text-base text-stone-900 placeholder-stone-400 shadow-sm transition-all focus:shadow-md focus:ring-2 focus:outline-none disabled:cursor-not-allowed disabled:opacity-60 dark:border-stone-800/60 dark:bg-stone-950 dark:text-stone-100"
  >
    <option value="">Select an assignee</option>
    {#each $studentStore as student (student.id)}
      <option
        value={JSON.stringify({
          assignee: student.id,
          studentTelegramId: student.studentTelegramId,
        })}
        selected={item.assignee === $user.id
          ? student.id === $assigneeStore
          : student.id === item.assignee}
      >
        {student.name}
      </option>
    {/each}
  </select>
</div>
