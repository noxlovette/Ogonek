<script lang="ts">
  import { formatDateOnly } from "$lib/utils";
  import CardClickable from "./CardClickable.svelte";
  import { user } from "$lib/stores";
  import Badge from "./Badge.svelte";
  import { Headline } from "../typography";
  import { getUrgency } from "$lib/utils";
  import SeenBadge from "./SeenBadge.svelte";
  import { VStack } from "../UI";

  let { task } = $props();
  const formattedDate: string = formatDateOnly(task.dueDate);
  const href: string =
    $user.role === "teacher" ? `/t/tasks/${task.id}` : `/s/tasks/${task.id}`;
  const badgeText: string = `Дедлайн ${formattedDate}`;
  const urgency = getUrgency(task.dueDate);
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
