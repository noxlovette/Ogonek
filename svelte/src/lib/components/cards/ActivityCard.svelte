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
      <BookOpen class=" " />
    {:else if activity.action.startsWith("updated")}
      <NotebookPen class="text-orange-600" />
    {:else if activity.action.startsWith("deleted")}
      <Trash2 class="text-red-600" />
    {/if}
  {:else if activity.modelType.startsWith("task")}
    {#if activity.action.startsWith("new")}
      <ListTodo class=" " />
    {:else if activity.action.startsWith("updated")}
      <SquarePen class=" text-orange-600" />
    {:else if activity.action.startsWith("deleted")}
      <Trash2 class=" text-red-600" />
    {:else if activity.action.startsWith("completed")}
      <CheckCircle2 class="text-green-600" />
    {/if}
  {:else if activity.modelType.startsWith("deck")}
    {#if activity.action.startsWith("new")}
      <WholeWord class=" " />
    {:else if activity.action.startsWith("updated")}
      <SquarePen class=" text-oramge-600" />
    {:else if activity.action.startsWith("deleted")}
      <Trash2 class="text-red-600"></Trash2>
    {:else if activity.action.startsWith("subscribed")}
      <Bell class=" text-green-600" />
    {:else if activity.action.startsWith("unsubscribed")}
      <BellOff class=" text-stone-400" />
    {/if}
  {:else}
    <Dot class=" " />
  {/if}
{/snippet}
<a
  class="ring-default bg-default flex items-center justify-between gap-4 rounded-xl p-4 shadow-sm transition-shadow hover:shadow-md"
  {href}
>
  <!-- Icon -->
  <div
    class="flex-shrink-0 rounded-full bg-stone-100 p-3 ring-1 ring-stone-300 ring-inset dark:bg-stone-800 dark:ring-stone-700"
  >
    {@render icon(activity)}
  </div>

  <!-- Content -->
  <div class="flex flex-1 flex-col overflow-hidden">
    <p
      class="truncate text-sm font-semibold text-stone-900 capitalize dark:text-stone-100"
    >
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
