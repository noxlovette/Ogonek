<script lang="ts">
  import { formatDateTime } from "$lib/utils";
  import Clickable from "./CardClickable.svelte";
  import { user, notification } from "$lib/stores";
  import { onMount } from "svelte";
  import { H2 } from "../typography";
  import { enhance } from "$app/forms";
  import { CheckSquare, Download, Loader2, Square } from "lucide-svelte";
  import { parseMarkdown } from "$lib/utils";

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
    class="inline-flex items-baseline justify-between space-x-8 text-lg md:text-xl lg:text-2xl xl:text-3xl"
  >
    <H2>
      {task.title}
    </H2>
    {#if interactive}
      <form
        class="flex"
        method="post"
        use:enhance={() => {
          return async ({ result }) => {
            if (result.type === "success") {
              const message = completed
                ? "Marked As Completed"
                : "Not Completed";
              notification.set({ message, type: "success" });
            } else {
              notification.set({
                message: "Failed to mark as completed",
                type: "error",
              });
            }
          };
        }}
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
    <p class:overdue class="text-milk-500 {overdue ? 'text-cacao-500' : ''}">
      Due {formattedDate}
    </p>
    {#if interactive && task.filePath}
      <a
        class="pointer-events-auto"
        href="/download/{task.filePath}"
        onclick={() => (isPreloading = true)}
      >
        {#if !isPreloading}
          <Download class="size-6" />
        {:else}
          <Loader2 class="animate-spin" />
        {/if}
      </a>
    {/if}
  </div>

  <p class="text-milk-600 text-sm/relaxed lg:text-base/relaxed">
    {@html rendered}
  </p>
</Clickable>
