<script lang="ts">
  import { formatDate } from "@noxlovette/svarog";
  import CardClickable from "./CardClickable.svelte";
  import { user } from "$lib/stores";
  import Badge from "./Badge.svelte";
  import { H3 } from "../typography";
  import { getUrgency } from "$lib/utils";
  import Priority from "./Priority.svelte";
  import SeenBadge from "./SeenBadge.svelte";
  import { m } from "$lib/paraglide/messages";

  let { task } = $props();
  const formattedDate: string = formatDate(task.dueDate);
  const href: string =
    $user.role === "teacher"
      ? `/t/tasks/t/${task.id}`
      : `/s/tasks/t/${task.id}`;
  const badgeText: string = `${m.less_arable_starfish_belong()} ${formattedDate}`;
  const urgency = getUrgency(task);
</script>

<CardClickable {href}>
  <div class="flex items-start justify-between gap-3">
    <H3>
      {task.title}
    </H3>
    <Priority priority={task.priority} />
  </div>
  <Badge {badgeText} {urgency} />
  <SeenBadge seen={task.seen} />
</CardClickable>
