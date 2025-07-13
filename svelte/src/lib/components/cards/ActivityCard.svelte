<!-- ActivityCard.svelte -->
<script lang="ts">
  import type { ActivityLog } from "$lib/types";
  import { page } from "$app/state";
  import { formatDateTime } from "@noxlovette/svarog";
  import {
    ActivityIcon,
    Bell,
    BellOff,
    BookOpen,
    CheckCircle2,
    Dot,
    ListTodo,
    NotebookPen,
    SquarePen,
    Trash2,
    WholeWord,
  } from "lucide-svelte";

  let { activity }: { activity: ActivityLog } = $props();

  const href = $derived.by(() => {
    let base;

    switch (activity.modelType) {
      case "lesson":
        base = "lessons/l";
        break;
      case "task":
        base = "tasks/t";
        break;
      case "deck":
        base = "words/w";
        break;
      default:
        base = "dashboard";
    }

    return `/${page.params.role}/${base}/${activity.modelId}`;
  });
</script>

{#snippet icon(activity: ActivityLog)}
  {#if activity.modelType.startsWith("lesson")}
    {#if activity.action.startsWith("new")}
      <BookOpen strokeWidth="1" class="size-5" />
    {:else if activity.action.startsWith("updated")}
      <NotebookPen strokeWidth="1" class="size-5 text-orange-600" />
    {:else if activity.action.startsWith("deleted")}
      <Trash2 strokeWidth="1" class="size-5 text-red-600" />
    {/if}
  {:else if activity.modelType.startsWith("task")}
    {#if activity.action.startsWith("new")}
      <ListTodo strokeWidth="1" class="size-5" />
    {:else if activity.action.startsWith("updated")}
      <SquarePen strokeWidth="1" class="size-5 text-orange-600" />
    {:else if activity.action.startsWith("deleted")}
      <Trash2 strokeWidth="1" class=" size-5 text-red-600" />
    {:else if activity.action.startsWith("completed")}
      <CheckCircle2 strokeWidth="1" class="size-5 text-green-600" />
    {/if}
  {:else if activity.modelType.startsWith("deck")}
    {#if activity.action.startsWith("new")}
      <WholeWord strokeWidth="1" class="size-5" />
    {:else if activity.action.startsWith("updated")}{:else if activity.action.startsWith("deleted")}
      <Trash2 strokeWidth="1" class="size-5 text-red-600"></Trash2>
    {:else if activity.action.startsWith("subscribed")}
      <Bell strokeWidth="1" class="size-5 text-green-600" />
    {:else if activity.action.startsWith("unsubscribed")}
      <BellOff strokeWidth="1" class="size-5 text-stone-400" />
    {/if}
  {:else}
    <Dot class=" " />
  {/if}
{/snippet}
<a
  class="bg-default flex items-center justify-between gap-3 rounded-xl border border-stone-200 p-2 shadow-sm transition-shadow hover:shadow-md dark:border-stone-800"
  {href}
>
  <!-- Icon -->
  <div
    class="flex-shrink-0 rounded-full bg-stone-100 p-2 ring-1 ring-stone-300 ring-inset dark:bg-stone-800 dark:ring-stone-700"
  >
    {@render icon(activity)}
  </div>

  <!-- Content -->
  <div class="flex flex-1 flex-col overflow-hidden">
    <p class="truncate text-sm font-semibold capitalize">
      {activity.action}
      <span class="ml-1 font-normal text-stone-600 dark:text-stone-400">
        {activity.modelType}
      </span>
    </p>

    {#if activity.createdAt}
      <time class="mt-1 text-xs text-stone-500 dark:text-stone-400">
        {formatDateTime(activity.createdAt)}
      </time>
    {/if}
  </div>
</a>
