<script lang="ts">
  import { formatDate } from "@noxlovette/svarog";
  import CardClickable from "./CardClickable.svelte";
  import { user } from "$lib/stores";
  import Badge from "./Badge.svelte";
  import { H3 } from "../typography";
  import { getUrgency } from "$lib/utils";

  let { task } = $props();

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
