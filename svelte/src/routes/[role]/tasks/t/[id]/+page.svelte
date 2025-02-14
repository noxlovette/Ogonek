<script lang="ts">
  import { H1, H2, ButtonRaw } from "$lib/components";
  import { user } from "$lib/stores";
  import { formatDateTime } from "$lib/utils";
  import { goto } from "$app/navigation";
  import { page } from "$app/state";
  import { notification } from "$lib/stores";
  import { enhance } from "$app/forms";
  import { Download, Loader2, CheckSquare, Square } from "lucide-svelte";

  let { data } = $props();
  const { task, rendered } = data;

  let role = $derived(page.params.role);
  let overdue = $state(false);
  let completed = $state(task.completed);

  let formattedDate = formatDateTime(task.createdAt);
  let isPreloading = $state(false);
</script>

<svelte:head>
  <title>Task From {formattedDate}</title>
</svelte:head>

{#if role === "t"}
  <div class="flex items-center justify-between">
    <div class="flex space-x-4">
      <H1>{task.title}</H1>
      <a
        href="/t/tasks/t/{task.id}/edit"
        class="bg-cacao-600 text-cacao-50 hover:bg-cacao-700 focus:ring-cacao-500 rounded-lg px-4 py-2 transition-colors focus:ring-2 focus:ring-offset-2 focus:outline-none disabled:opacity-50"
        >Edit</a
      >
    </div>
    <div class="text-right">
      <p class="text-milk-700 block font-medium">Student</p>
      <h3 class="">
        {#if task.assigneeName === $user.username}
          Not Assigned
        {:else}
          {task.assigneeName}
        {/if}
      </h3>
    </div>
  </div>
  <H2>Content</H2>
  <div class="markdown ring-milk-200 dark:ring-milk-900 rounded-lg p-4 ring-2">
    {@html rendered}
  </div>

  {#if task.filePath}
    <div>
      <ButtonRaw
        onclick={() => goto(`/download/${task.filePath}`)}
        buttonName="Download"
      />
    </div>
  {/if}
{:else}
  <div class="flex items-baseline justify-between">
    <div class="flex items-baseline space-x-4">
      <H1>{task.title}</H1>

      {#if task.filePath}
        <a
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
    </div>
  </div>
  <div class="markdown ring-milk-200 dark:ring-milk-900 rounded-lg p-4 ring-2">
    {@html rendered}
  </div>
{/if}
