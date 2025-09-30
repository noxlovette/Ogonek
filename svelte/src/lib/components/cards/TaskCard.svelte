<script lang="ts">
  import { formatDate } from "$lib/utils";
  import CardClickable from "./CardClickable.svelte";
  import { user } from "$lib/stores";
  import Badge from "./Badge.svelte";
  import { Headline } from "../typography";
  import { getUrgency } from "$lib/utils";
  import SeenBadge from "./SeenBadge.svelte";
  import { m } from "$lib/paraglide/messages";
  import { VStack } from "../UI";

  let { task } = $props();
  const formattedDate: string = formatDate(task.dueDate);
  const href: string =
    $user.role === "teacher" ? `/t/tasks/${task.id}` : `/s/tasks/${task.id}`;
  const badgeText: string = `${m.less_arable_starfish_belong()} ${formattedDate}`;
  const urgency = getUrgency(task);
</script>

<CardClickable dataCy="task-card" {href}>
  <Headline>
    {task.title}
  </Headline>
  <VStack>
    <Badge {urgency}>{badgeText}</Badge>
  </VStack>
  <SeenBadge seen={task.seen} />
</CardClickable>
