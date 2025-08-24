<script lang="ts">
  import { formatDate } from "@noxlovette/svarog";
  import CardClickable from "./CardClickable.svelte";
  import { user } from "$lib/stores";
  import Badge from "./Badge.svelte";
  import { Headline } from "../typography";
  import { getUrgency } from "$lib/utils";
  import Priority from "./Priority.svelte";
  import SeenBadge from "./SeenBadge.svelte";
  import { m } from "$lib/paraglide/messages";

  let { task } = $props();
  const formattedDate: string = formatDate(task.dueDate);
  const href: string =
    $user.role === "teacher" ? `/t/tasks/${task.id}` : `/s/tasks/${task.id}`;
  const badgeText: string = `${m.less_arable_starfish_belong()} ${formattedDate}`;
  const urgency = getUrgency(task);
</script>

<CardClickable {href}>
  <div class="flex items-start justify-between gap-3">
    <Headline>
      {task.title}
    </Headline>
    <Priority priority={task.priority} />
  </div>
  <Badge {badgeText} {urgency} />
  <SeenBadge seen={task.seen} />
</CardClickable>
