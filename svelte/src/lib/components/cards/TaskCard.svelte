<script lang="ts">
  import { formatDate } from "@noxlovette/svarog";
  import CardClickable from "./CardClickable.svelte";
  import { user } from "$lib/stores";
  import type { Task, UrgencyLevel } from "$lib/types";
  import Badge from "./Badge.svelte";
  import { H3 } from "../typography";

  let { task } = $props();
  function getUrgency(task: Task): UrgencyLevel {
    const now = new Date();
    const due = new Date(task.dueDate);
    const diffDays = Math.ceil(
      (due.getTime() - now.getTime()) / (1000 * 60 * 60 * 24),
    );

    if (diffDays < 1) return "overdue";
    if (diffDays <= 1) return "urgent";
    if (diffDays <= 3) return "soon";
    return "normal";
  }

  const formattedDate: string = formatDate(task.dueDate);
  const href: string =
    $user.role === "teacher"
      ? `/t/tasks/t/${task.id}`
      : `/s/tasks/t/${task.id}`;

  const badgeText: string = `Due ${formattedDate}`;
  const urgency = getUrgency(task);
</script>

<CardClickable {href}>
  <H3>
    {task.title}
  </H3>
  <Badge {badgeText} {urgency} />
</CardClickable>
