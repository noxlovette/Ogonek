<script lang="ts">
  import { formatDateTime } from "$lib/utils";
  import Clickable from "./CardClickable.svelte";
  import { user } from "$lib/stores";
  import { onMount } from "svelte";
  import { H2 } from "../typography";
  import { enhance } from "$app/forms";
  import { CheckSquare, Square } from "lucide-svelte";
  import { parseMarkdown } from "$lib/utils";
  import { enhanceForm } from "$lib/utils";
  import { invalidate } from "$app/navigation";

  onMount(async () => {
    rendered = await parseMarkdown(task.markdown);
    overdue = new Date(task.dueDate) < new Date();
  });
  let isPreloading = $state(false);
  let { task, interactive = false } = $props();
  let overdue = $state(false);
  let rendered = $state(task.markdown);
  const formattedDate = formatDateTime(task.dueDate);
  let completed = $state(task.completed);
  let href =
    $user.role === "teacher"
      ? `/t/tasks/t/${task.id}`
      : `/s/tasks/t/${task.id}`;
</script>

<Clickable {href}>
  <div
    id="task-header"
    class="flex items-baseline justify-between text-lg md:text-xl lg:text-2xl xl:text-3xl"
  >
    <H2>
      {task.title}
    </H2>
    {#if interactive}
      <form
        class="flex"
        method="post"
        action="?/completed"
        use:enhance={enhanceForm({
          messages: {
            success: completed ? "Not Completed" : "Marked As Completed",
            defaultError: "Failed to mark as completed",
          },
          handlers: {
            success: async () => {
              invalidate("tasks:completed");
            },
          },
        })}
      >
        <button
          onclick={() => (completed = !completed)}
          class="pointer-events-auto"
          class:overdue
        >
          {#if completed}
            <CheckSquare class="h-6 w-6" />
          {:else}
            <Square class="h-6 w-6" />
          {/if}
        </button>
        <input type="hidden" name="completed" value={completed} />
        <input type="hidden" name="id" value={task.id} />
      </form>
    {/if}
  </div>

  <div class="mt-auto flex items-center justify-between pt-4 text-sm/tight">
    <p class:overdue class="text-stone-500 {overdue ? 'text-cacao-500' : ''}">
      Due {formattedDate}
    </p>
  </div>

  <p class="text-sm/relaxed text-stone-600 lg:text-base/relaxed">
    {@html rendered}
  </p>
</Clickable>
