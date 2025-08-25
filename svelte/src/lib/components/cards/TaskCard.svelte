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
  import { HStack, VStack } from "../UI";
  import Divider from "../UI/toolbar/Divider.svelte";

  let { task } = $props();
  const formattedDate: string = formatDate(task.dueDate);
  const href: string =
    $user.role === "teacher" ? `/t/tasks/${task.id}` : `/s/tasks/${task.id}`;
  const badgeText: string = `${m.less_arable_starfish_belong()} ${formattedDate}`;
  const urgency = getUrgency(task);
</script>

<CardClickable {href}>
  <Headline>
    {task.title}
  </Headline>
  <VStack>
    <Badge {urgency}>{badgeText}</Badge>
    <Divider />
    <Priority priority={task.priority} />
  </VStack>
  <SeenBadge seen={task.seen} />
</CardClickable>
