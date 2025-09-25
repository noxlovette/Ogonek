<script lang="ts">
  import type { ActivityLog } from "$lib/types";
  import { page } from "$app/state";

  import {
    Bell,
    BellOff,
    BookOpen,
    CheckCircle2,
    Dot,
    ListTodo,
    Trash2,
    WholeWord,
  } from "lucide-svelte";
  import { formatDateTime } from "$lib/utils";

  let { activity }: { activity: ActivityLog } = $props();

  const href = $derived.by(() => {
    let base;

    switch (activity.modelType) {
      case "lesson":
        base = "lessons";
        break;
      case "task":
        base = "tasks";
        break;
      case "deck":
        base = "flashcards";
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
      <BookOpen
        strokeWidth="1"
        class="size-5 text-amber-600 dark:text-amber-400"
      />
    {:else if activity.action.startsWith("deleted")}
      <Trash2 strokeWidth="1" class="size-5 text-rose-600 dark:text-rose-400" />
    {/if}
  {:else if activity.modelType.startsWith("task")}
    {#if activity.action.startsWith("new")}
      <ListTodo strokeWidth="1" class="size-5" />
    {:else if activity.action.startsWith("updated")}
      <ListTodo
        strokeWidth="1"
        class="size-5 text-amber-600 dark:text-amber-400"
      />
    {:else if activity.action.startsWith("deleted")}
      <Trash2
        strokeWidth="1"
        class=" size-5 text-rose-600 dark:text-rose-400"
      />
    {:else if activity.action.startsWith("completed")}
      <CheckCircle2 strokeWidth="1" class="size-5 text-emerald-400" />
    {/if}
  {:else if activity.modelType.startsWith("deck")}
    {#if activity.action.startsWith("new")}
      <WholeWord strokeWidth="1" class="size-5" />
    {:else if activity.action.startsWith("updated")}
      <WholeWord
        strokeWidth="1"
        class="size-5 text-amber-600 dark:text-amber-400"
      ></WholeWord>
    {:else if activity.action.startsWith("deleted")}
      <Trash2 strokeWidth="1" class="size-5 text-rose-600 dark:text-rose-400"
      ></Trash2>
    {:else if activity.action.startsWith("subscribed")}
      <Bell strokeWidth="1" class="size-5 text-emerald-400" />
    {:else if activity.action.startsWith("unsubscribed")}
      <BellOff strokeWidth="1" class="size-5 text-stone-400" />
    {/if}
  {:else}
    <Dot class=" " />
  {/if}
{/snippet}
<a
  class="bg-default ring-default flex items-center justify-between gap-3 rounded-2xl p-2 shadow-md"
  {href}
>
  <!-- Icon -->

  {@render icon(activity)}

  <!-- Content -->
  <div class="flex flex-1 flex-col overflow-hidden">
    <p
      class="truncate text-sm font-semibold text-stone-600 capitalize dark:text-stone-400"
    >
      {activity.action}
      <span class="ml-1 font-normal text-stone-600">
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
